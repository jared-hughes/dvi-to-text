// Treat the entire input as a single batch
// TODO: stream the input, stdin or something
fn text(dvi_bytes: &[u8]) {
    let mut bytes_remaining = dvi_bytes;
    while bytes_remaining.len() > 0 {
        match dvi::Instruction::parse(bytes_remaining) {
            Err(e) => eprintln!("Error parsing DVI file: {e:?}"),
            Ok((b, inst)) => {
                use dvi::Instruction::*;
                match inst {
                    Set(charcode) | Put(charcode) => println!("Typeset char {charcode:x}"),
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
}

/// temporary usage: cargo run ../tex-output/long.dvi
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments.");
        println!("Usage: dvi-to-text filename.dvi");
        return;
    }
    let path = &args[1];
    match std::fs::read(path) {
        Err(e) => eprintln!("Error reading DVI file: {e:?}"),
        Ok(bytes) => {
            println!("Loaded a dvi file with {} bytes", bytes.len());
            text(bytes.as_slice());
        }
    }
}
