use mp4box::hex_range;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn temp_file(bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join("mp4box_hex_range_test.bin");
    let mut f = File::create(&path).unwrap();
    f.write_all(bytes).unwrap();
    path
}

#[test]
fn hex_range_reads_within_bounds() {
    let data = (0u8..64u8).collect::<Vec<_>>();
    let path = temp_file(&data);

    let dump = hex_range(&path, 16, 16).expect("hex_range failed");

    assert_eq!(dump.offset, 16);
    assert_eq!(dump.length, 16);
    // sanity: first byte of region is 16
    assert!(dump.hex.contains("10"));
}

#[test]
fn hex_range_clamps_to_eof() {
    let data = (0u8..32u8).collect::<Vec<_>>();
    let path = temp_file(&data);

    // ask past EOF
    let dump = hex_range(&path, 24, 32).expect("hex_range failed");

    // we only have 8 bytes from 24..32
    assert_eq!(dump.offset, 24);
    assert_eq!(dump.length, 8);
}
