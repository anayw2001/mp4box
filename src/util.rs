use std::io::{Read, Seek, SeekFrom};

pub fn read_slice<R: Read + Seek>(r: &mut R, offset: u64, len: u64) -> std::io::Result<Vec<u8>> {
    r.seek(SeekFrom::Start(offset))?;
    let mut v = vec![0u8; len as usize];
    r.read_exact(&mut v)?;
    Ok(v)
}

pub fn hex_dump(bytes: &[u8], start_offset: u64) -> String {
    // Simple hexdump
    let mut out = String::new();
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offs = start_offset + (i as u64) * 16;
        let hexs: String = chunk.iter().map(|b| format!("{:02x} ", b)).collect();
        let ascii: String = chunk.iter().map(|b| {
            let c = *b;
            if (32..=126).contains(&c) { c as char } else { '.' }
        }).collect();
        out.push_str(&format!("{:08x}  {:<48}  |{}|\n", offs, hexs, ascii));
    }
    out
}
