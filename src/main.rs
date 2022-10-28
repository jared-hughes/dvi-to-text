use std::fs;

fn main() {
    match fs::read("../tex-output/long.dvi") {
        Err(e) => eprintln!("Error reading DVI file: {e:?}"),
        Ok(bytes) => {
                    println!("Loaded a dvi file with {} bytes", bytes.len());
            match dvi::Instruction::parse(bytes.as_slice()) {
                Err(e) => eprintln!("Error parsing DVI file: {e:?}"),
                Ok(x) => {
                    println!("After first parse, {} bytes", x.0.len());
                }
            }
        }
    }
}
