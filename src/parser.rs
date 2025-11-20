use crate::boxes::{BoxHeader, BoxRef, FourCC, NodeKind};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid box size")]
    InvalidSize,
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub fn read_box_header<R: Read + Seek>(r: &mut R) -> Result<BoxHeader> {
    let start = r.stream_position()?;
    let size32 = r.read_u32::<BigEndian>()?;
    let mut typ = [0u8; 4]; r.read_exact(&mut typ)?;
    let mut size = size32 as u64;

    if size32 == 1 {
        size = r.read_u64::<BigEndian>()?;
    }

    let mut uuid = None;
    if &typ == b"uuid" {
        let mut u = [0u8; 16];
        r.read_exact(&mut u)?;
        uuid = Some(u);
    }

    let header_size = match (size32 == 1, &typ == b"uuid") {
        (true, true)  => 8 + 8 + 16,
        (true, false) => 8 + 8,
        (false, true) => 8 + 16,
        (false, false)=> 8,
    } as u64;

    if size != 0 && size < header_size {
        return Err(ParseError::InvalidSize);
    }

    Ok(BoxHeader { size, typ: FourCC(typ), uuid, header_size, start })
}

pub fn parse_children<R: Read + Seek>(r: &mut R, parent_end: u64) -> Result<Vec<BoxRef>> {
    let mut kids = Vec::new();
    while r.stream_position()? < parent_end {
        let h = read_box_header(r)?;
        let box_end = if h.size == 0 { parent_end } else { h.start + h.size };

        // Decide kind
        let kind = if is_container(&h) {
            // recurse into container
            let content_start = h.start + h.header_size;
            r.seek(SeekFrom::Start(content_start))?;
            let child = parse_children(r, box_end)?;
            NodeKind::Container(child)
        } else if is_full_box(&h) {
            let content_start = h.start + h.header_size;
            r.seek(SeekFrom::Start(content_start))?;
            let version = r.read_u8()?;
            let mut f = [0u8;3]; r.read_exact(&mut f)?;
            let flags = ((f[0] as u32) << 16) | ((f[1] as u32) << 8) | (f[2] as u32);
            let data_offset = r.stream_position()?;
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

        // Skip to end of box
        r.seek(SeekFrom::Start(box_end))?;
        kids.push(BoxRef { hdr: h, kind });
    }
    Ok(kids)
}

// Known containers from ISOBMFF / MP4
fn is_container(h: &BoxHeader) -> bool {
    matches!(&h.typ.0,
        b"moov" | b"trak" | b"mdia" | b"minf" | b"stbl" | b"edts" |
        b"udta" | b"meta" | b"moof" | b"traf" | b"mfra" | b"sinf" |
        b"ipro" | b"schi" | b"dinf" | b"iprp" | b"iloc" /* (full, but has kids via ipco) */ |
        b"tref" | b"meco" | b"mere" | b"iref" | b"pitm" /* (full) */ |
        b"mvex" | b"stsd" /* stsd is a full box but contains sample entries (children) */
    )
}

// "FullBox" (version+flags) types (non-exhaustive; safe default is to treat unknown as Leaf/Unknown)
fn is_full_box(h: &BoxHeader) -> bool {
    matches!(&h.typ.0,
        b"mvhd" | b"tkhd" | b"mdhd" | b"hdlr" | b"vmhd" | b"smhd" |
        b"nmhd" | b"dref" | b"stsd" | b"stts" | b"ctts" | b"stsc" |
        b"stsz" | b"stz2" | b"stco" | b"co64" | b"stss" | b"stsh" |
        b"elst" | b"url " | b"urn " | b"tfhd" | b"trun" | b"mfhd" |
        b"tfdt" | b"mehd" | b"trex" | b"pssh" | b"sidx" | b"mdat" /* not actually FullBox, will be Leaf, but harmless if absent here */
    ) && &h.typ.0 != b"stsd" // stsd is FullBox but special-cased as container above
}
