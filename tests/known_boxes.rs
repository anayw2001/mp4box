use mp4box::boxes::FourCC;
use mp4box::known_boxes::KnownBox;

#[test]
fn known_box_from_ftyp() {
    let cc = FourCC(*b"ftyp");
    let kb = KnownBox::from(cc);
    assert!(matches!(kb, KnownBox::Ftyp));
    assert_eq!(kb.full_name(), "File Type Box");
}

#[test]
fn known_box_classifies_container() {
    let moov = KnownBox::from(FourCC(*b"moov"));
    assert!(moov.is_container());

    let ftyp = KnownBox::from(FourCC(*b"ftyp"));
    assert!(!ftyp.is_container());
}

#[test]
fn known_box_classifies_full_box() {
    let mvhd = KnownBox::from(FourCC(*b"mvhd"));
    assert!(mvhd.is_full_box());

    let mdat = KnownBox::from(FourCC(*b"mdat"));
    assert!(!mdat.is_full_box());
}
