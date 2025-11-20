use clap::{ArgAction, Parser};
use mp4box::{
    boxes::{BoxKey, FourCC, NodeKind, BoxRef},
    parser::{parse_children, read_box_header},
    registry::{FtypDecoder, Registry},
    util::{hex_dump, read_slice},
};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Parser, Debug)]
#[command(version, about = "Minimal MP4/ISOBMFF box explorer")]
struct Args {
    /// MP4/ISOBMFF file path
    path: String,

    /// Dump raw payload of this 4CC (e.g. --raw stsd) or uuid:xxxxxxxx...
    #[arg(long = "raw")]
    raw: Option<String>,

    /// Limit recursion depth
    #[arg(long, default_value_t = 64)]
    max_depth: usize,

    /// Print structured values when a decoder exists
    #[arg(long, action=ArgAction::SetTrue)]
    decode: bool,

    /// Show bytes count when dumping raw (0 means entire box payload)
    #[arg(long, default_value_t = 0)]
    bytes: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut f = File::open(&args.path)?;

    let file_len = f.metadata()?.len();
    let top = {
        // Top-level loop
        let mut kids = Vec::new();
        while f.stream_position()? < file_len {
            let h = read_box_header(&mut f)?;
            let box_end = if h.size == 0 { file_len } else { h.start + h.size };

            // Try to parse subtree (container full parse), else leaf
            let kind = if is_container(&h) {
                f.seek(SeekFrom::Start(h.start + h.header_size))?;
                NodeKind::Container(parse_children(&mut f, box_end)?)
            } else if is_full_box(&h) {
                f.seek(SeekFrom::Start(h.start + h.header_size))?;
                // Reuse parse_children to get FullBox; simplest is call again
                // use mp4box::parser::read_box_header as _; // silence warn
                // But we want FullBox meta: read version+flags inline:
                use byteorder::{ReadBytesExt};
                let version = f.read_u8()?;
                let mut fl = [0u8;3]; f.read_exact(&mut fl)?;
                let flags = ((fl[0] as u32) << 16) | ((fl[1] as u32) << 8) | (fl[2] as u32);
                let data_offset = f.stream_position()?;
                let data_len = box_end.saturating_sub(data_offset);
                NodeKind::FullBox { version, flags, data_offset, data_len }
            } else {
                let data_offset = h.start + h.header_size;
                let data_len = box_end.saturating_sub(data_offset);
                if &h.typ.0 == b"uuid" {
                    NodeKind::Unknown { data_offset, data_len }
                } else {
                    NodeKind::Leaf { data_offset, data_len }
                }
            };
            f.seek(SeekFrom::Start(box_end))?;
            kids.push(mp4box::boxes::BoxRef { hdr: h, kind });
        }
        kids
    };

    let reg = Registry::new()
        .with_decoder(BoxKey::FourCC(FourCC(*b"ftyp")), "ftyp", Box::new(FtypDecoder));

    // Printing tree
    for b in &top {
        print_box(&mut f, b, 0, args.max_depth, args.decode, &reg)?;
    }

    // Optional raw dump
    if let Some(sel) = args.raw.as_ref() {
        dump_raw(&mut f, &top, sel, args.bytes)?;
    }

    Ok(())
}

fn print_box(
    f: &mut File,
    b: &mp4box::boxes::BoxRef,
    depth: usize,
    max_depth: usize,
    decode: bool,
    reg: &Registry,
) -> anyhow::Result<()> {
    let indent = "  ".repeat(depth);
    let hdr = &b.hdr;
    match &b.kind {
        NodeKind::FullBox { version, flags, .. } => {
            println!("{indent}{:>6} {:>10} {} (ver={}, flags=0x{:06x})",
                format!("{:#x}", hdr.start),
                hdr.size,
                display_type(hdr),
                version, flags
            );
            if decode {
                maybe_decode(f, b, reg)?;
            }
        }
        NodeKind::Leaf { .. } | NodeKind::Unknown { .. } => {
            println!("{indent}{:>6} {:>10} {}",
                format!("{:#x}", hdr.start), hdr.size, display_type(hdr));
            if decode {
                maybe_decode(f, b, reg)?;
            }
        }
        NodeKind::Container(children) => {
            println!("{indent}{:>6} {:>10} {} (container)",
                format!("{:#x}", hdr.start), hdr.size, display_type(hdr));
            if depth + 1 <= max_depth {
                for c in children {
                    print_box(f, c, depth + 1, max_depth, decode, reg)?;
                }
            }
        }
    }
    Ok(())
}

fn display_type(h: &mp4box::boxes::BoxHeader) -> String {
    if &h.typ.0 == b"uuid" {
        let u = h.uuid.unwrap_or([0u8;16]);
        format!("uuid:{:02x?}", u)
    } else {
        h.typ.to_string()
    }
}

fn maybe_decode(f: &mut File, b: &BoxRef, reg: &Registry) -> anyhow::Result<()> {
    use std::io::{Seek, SeekFrom};

    let key = if &b.hdr.typ.0 == b"uuid" {
        BoxKey::Uuid(b.hdr.uuid.unwrap())
    } else {
        BoxKey::FourCC(b.hdr.typ)
    };

    // Recompute offset/len for safety
    let (off, len) = match &b.kind {
        // For FullBox we trust stored offsets (already after version+flags)
        NodeKind::FullBox { data_offset, data_len, .. } => (*data_offset, *data_len),

        // For Leaf/Unknown, derive payload from header
        NodeKind::Leaf { .. } | NodeKind::Unknown { .. } => {
            let hdr = &b.hdr;
            if hdr.size == 0 {
                // extends to parent; we don't know the end here -> skip decoding
                return Ok(());
            }
            let off = hdr.start + hdr.header_size;
            let len = hdr.size.saturating_sub(hdr.header_size);
            if len == 0 {
                return Ok(());
            }
            (off, len)
        }

        NodeKind::Container(_) => {
            // containers don't have direct payload
            return Ok(());
        }
    };

    if len == 0 {
        return Ok(());
    }

    f.seek(SeekFrom::Start(off))?;
    let mut limited = f.take(len);

    if let Some(res) = reg.decode(&key, &mut limited, &b.hdr) {
        match res? {
            mp4box::registry::BoxValue::Text(s) => println!("        -> {}", s),
            mp4box::registry::BoxValue::Bytes(bytes) => println!("        -> {} bytes", bytes.len()),
        }
    }

    f.seek(SeekFrom::Start(off + len))?;
    Ok(())
}



fn dump_raw(f: &mut File, boxes: &[mp4box::boxes::BoxRef], sel: &str, limit: usize) -> anyhow::Result<()> {
    let mut matches = Vec::new();
    select_boxes(boxes, sel, &mut matches);
    for (i, (off, len, hdr)) in matches.into_iter().enumerate() {
        let to_read = if limit == 0 || limit as u64 > len { len } else { limit as u64 };
        let data = read_slice(f, off, to_read)?;
        println!("\n== Dump {} ({}) payload: offset={:#x}, len={} ==", i, display_type(&hdr), off, to_read);
        print!("{}", hex_dump(&data, off));
    }
    Ok(())
}

fn select_boxes<'a>(
    list: &'a [mp4box::boxes::BoxRef],
    sel: &str,
    out: &mut Vec<(u64, u64, mp4box::boxes::BoxHeader)>
) {
    use mp4box::boxes::{BoxHeader, FourCC, NodeKind};

    for b in list {
        let matches_sel = if let Some(u) = b.hdr.uuid {
            if sel.starts_with("uuid:") {
                let hex = sel.trim_start_matches("uuid:").to_ascii_lowercase();
                let flat: String = u.iter().map(|x| format!("{:02x}", x)).collect();
                flat.starts_with(&hex)
            } else {
                false
            }
        } else if sel.len() == 4 {
            b.hdr.typ == FourCC::from_str(sel).unwrap()
        } else {
            false
        };

        if matches_sel {
            match &b.kind {
                NodeKind::FullBox { data_offset, data_len, .. } => {
                    out.push((*data_offset, *data_len, b.hdr.clone()));
                }
                NodeKind::Leaf { data_offset, data_len } => {
                    out.push((*data_offset, *data_len, b.hdr.clone()));
                }
                NodeKind::Unknown { data_offset, data_len } => {
                    out.push((*data_offset, *data_len, b.hdr.clone()));
                }
                NodeKind::Container(_) => {
                    // Treat the container's *payload* as raw region:
                    let hdr: &BoxHeader = &b.hdr;
                    if hdr.size != 0 && hdr.size > hdr.header_size {
                        let off = hdr.start + hdr.header_size;
                        let len = hdr.size - hdr.header_size;
                        out.push((off, len, hdr.clone()));
                    }
                }
            }
        }

        if let NodeKind::Container(kids) = &b.kind {
            select_boxes(kids, sel, out);
        }
    }
}


// local copies to avoid making parser::is_* public
fn is_container(h: &mp4box::boxes::BoxHeader) -> bool {
    matches!(&h.typ.0,
        b"moov" | b"trak" | b"mdia" | b"minf" | b"stbl" | b"edts" |
        b"udta" | b"meta" | b"moof" | b"traf" | b"mfra" | b"sinf" |
        b"ipro" | b"schi" | b"dinf" | b"iprp" | b"tref" | b"meco" | b"iref" | b"mvex" | b"stsd"
    )
}
fn is_full_box(h: &mp4box::boxes::BoxHeader) -> bool {
    matches!(&h.typ.0,
        b"mvhd" | b"tkhd" | b"mdhd" | b"hdlr" | b"vmhd" | b"smhd" |
        b"nmhd" | b"dref" | b"stts" | b"ctts" | b"stsc" |
        b"stsz" | b"stz2" | b"stco" | b"co64" | b"stss" | b"stsh" |
        b"elst" | b"url " | b"urn " | b"tfhd" | b"trun" | b"mfhd" |
        b"tfdt" | b"mehd" | b"trex" | b"pssh" | b"sidx"
    )
}
