use dvi::Instruction::{self, *};
use std::collections::HashMap;

// Treat the entire input as a single batch
// TODO: stream the input, stdin or something
pub fn text(dvi_bytes: &[u8]) -> Vec<u8> {
    let mut bytes_remaining = dvi_bytes;
    let mut output = Vec::<u8>::new();
    let mut fonts = HashMap::new();
    let mut curr_font: Option<dvi::FontDef> = None;
    while bytes_remaining.len() > 0 {
        let (b, inst) =
            Instruction::parse(bytes_remaining).expect("Bytes should be a valid DVI file");
        match inst {
            Set(charcode) | Put(charcode) => output.push(charcode.try_into().unwrap()),
            Eop | Y(_) | Z(_) => output.push(b'\n'),
            Right(x) => {
                if x > 0 {
                    let design_size = curr_font
                        .clone()
                        .expect("Font should be defined")
                        .design_size;
                    // division is equivalent to pseudocode:
                    // let space_width = (design_size/3)
                    // let num_spaces = round(float_div(x / space_width))
                    let num_spaces = ((x as u32) * 3 + design_size / 2) / design_size;
                    for _ in 0..num_spaces {
                        output.push(b' ');
                    }
                }
            }
            FontDef(font_def) => {
                fonts.insert(font_def.number, font_def);
            }
            Font(k) => {
                curr_font = Some(
                    fonts
                        .get(&k)
                        .expect("Font is defined before it is used")
                        .clone(),
                );
            }
            // ignoring horizontal movement, font changes, ...
            _ => {}
        }
        bytes_remaining = b;
    }
    output
}
