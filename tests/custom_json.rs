use anyhow::anyhow;
use mp4box::boxes::{BoxHeader, BoxKey, FourCC};
use mp4box::get_boxes;
use mp4box::registry::{BoxDecoder, BoxValue};
use serde_json::json;
use std::io::{Cursor, Read};

struct CustomJsonDecoder;

impl BoxDecoder for CustomJsonDecoder {
    fn decode(
        &self,
        _r: &mut dyn Read,
        _hdr: &BoxHeader,
        _version: Option<u8>,
        _flags: Option<u32>,
    ) -> anyhow::Result<BoxValue> {
        Ok(BoxValue::Json(json!({
            "foo": "bar",
            "baz": 123
        })))
    }
}

#[test]
fn test_custom_json_decoder() -> anyhow::Result<()> {
    // Create a mock MP4 with one 'custom' box
    // Header: size=12, type='cust'
    let data = vec![
        0, 0, 0, 12, // size
        b'c', b'u', b's', b't', // type
        0, 1, 2, 3, // payload
    ];
    let mut cursor = Cursor::new(data);
    let size = 12;

    let boxes = get_boxes(&mut cursor, size, true, |reg| {
        reg.with_decoder(
            BoxKey::FourCC(FourCC(*b"cust")),
            "custom",
            Box::new(CustomJsonDecoder),
        )
    })?;

    assert_eq!(boxes.len(), 1);
    let b = &boxes[0];
    assert_eq!(b.typ, "cust");

    // Check structured_data
    if let Some(v) = &b.json {
        assert_eq!(v["foo"], "bar");
        assert_eq!(v["baz"], 123);
    } else {
        return Err(anyhow!("JSON data not available"));
    }

    Ok(())
}
