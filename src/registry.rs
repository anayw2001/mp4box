use crate::boxes::{BoxHeader, BoxKey};
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, Clone)]
pub enum BoxValue {
    // You can add structured outputs for known boxes as needed
    Text(String),
    Bytes(Vec<u8>),
}

pub trait BoxDecoder: Send + Sync {
    fn decode(&self, r: &mut dyn Read, hdr: &BoxHeader) -> anyhow::Result<BoxValue>;
}

pub struct Registry {
    map: HashMap<BoxKey, BoxDecoderEntry>,
}

struct BoxDecoderEntry {
    inner: Box<dyn BoxDecoder>,
    _name: String,
}

impl Registry {
    pub fn new() -> Self { Self { map: HashMap::new() } }

    pub fn with_decoder(mut self, key: BoxKey, name: &str, dec: Box<dyn BoxDecoder>) -> Self {
        self.map.insert(key, BoxDecoderEntry { inner: dec, _name: name.to_string() });
        self
    }

    pub fn decode(&self, key: &BoxKey, r: &mut dyn Read, hdr: &BoxHeader) -> Option<anyhow::Result<BoxValue>> {
        self.map.get(key).map(|d| d.inner.decode(r, hdr))
    }
}

// Example: a trivial decoder for 'ftyp' that shows major+minor brands
pub struct FtypDecoder;

impl BoxDecoder for FtypDecoder {
    fn decode(&self, r: &mut dyn Read, _hdr: &BoxHeader) -> anyhow::Result<BoxValue> {
        // use byteorder::{BigEndian, ReadBytesExt};

        // Read the entire payload of this box (already limited by `Take` in maybe_decode)
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        if buf.len() < 8 {
            // Not enough data for major + minor; don't error hard, just report what we can.
            return Ok(BoxValue::Text(format!(
                "ftyp: payload too short ({} bytes)",
                buf.len()
            )));
        }

        let major = &buf[0..4];
        let minor = {
            let mut m = [0u8; 4];
            m.copy_from_slice(&buf[4..8]);
            u32::from_be_bytes(m)
        };

        let mut brands = Vec::new();
        for chunk in buf[8..].chunks(4) {
            if chunk.len() == 4 {
                brands.push(String::from_utf8_lossy(chunk).to_string());
            }
        }

        Ok(BoxValue::Text(format!(
            "major={} minor={} compatible={:?}",
            String::from_utf8_lossy(major),
            minor,
            brands
        )))
    }
}
