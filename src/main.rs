use std::io::Write;
mod lib;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect number of arguments.");
        eprintln!("Usage: dvi-to-text filename.dvi");
        return;
    }
    let path = &args[1];
    let bytes = std::fs::read(path).expect("Given file is readable");
    let bytes_out = lib::text(bytes.as_slice());
    let mut stdout = std::io::stdout();
    stdout.write_all(bytes_out.as_slice()).unwrap();
    stdout.flush().unwrap();
}
