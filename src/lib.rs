// Treat the entire input as a single batch
// TODO: stream the input, stdin or something
pub fn text(dvi_bytes: &[u8]) -> Vec<u8> {
    let mut bytes_remaining = dvi_bytes;
    let mut output = Vec::<u8>::new();
    while bytes_remaining.len() > 0 {
        match dvi::Instruction::parse(bytes_remaining) {
            Err(e) => eprintln!("Error parsing DVI file: {e:?}"),
            Ok((b, inst)) => {
                use dvi::Instruction::*;
                match inst {
                    Set(charcode) | Put(charcode) => output.push(charcode.try_into().unwrap()),
                    Bop(_c, _p) => println!("Beginning of page"),
                    Eop => println!("Ending of page"),
                    Down(y) => println!("Down by {y}"),
                    Y(y) => println!("Down by {y:?} and set y spacing"),
                    // ignore horizontal movement, font changes, ...
                    _ => {}
                }
                bytes_remaining = b;
            }
        }
    }
    output
}
