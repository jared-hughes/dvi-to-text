mod lib;

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
            lib::text(bytes.as_slice());
        }
    }
}
