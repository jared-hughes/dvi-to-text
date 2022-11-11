// Treat the entire input as a single batch
// TODO: stream the input, stdin or something
pub fn text(dvi_bytes: &[u8]) -> Vec<u8> {
    let mut bytes_remaining = dvi_bytes;
    let mut output = Vec::<u8>::new();
    while bytes_remaining.len() > 0 {
        let (b, inst) =
            dvi::Instruction::parse(bytes_remaining).expect("Bytes should be a valid DVI file");
        use dvi::Instruction::*;
        match inst {
            Set(charcode) | Put(charcode) => output.push(charcode.try_into().unwrap()),
            Bop(_c, _p) => eprintln!("Beginning of page"),
            Eop => eprintln!("Ending of page"),
            Down(y) => eprintln!("Down by {y}"),
            Y(y) => eprintln!("Down by {y:?} and set y spacing"),
            // ignoring horizontal movement, font changes, ...
            _ => {}
        }
        bytes_remaining = b;
    }
    output
}
