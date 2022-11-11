use dvi_to_text;

#[test]
fn abc() {
    let bytes = std::fs::read("tests/abc.dvi").expect("File is readable");
    assert_eq!(&dvi_to_text::text(bytes.as_slice())[..], "abc".as_bytes());
}
