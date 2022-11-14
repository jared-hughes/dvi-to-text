use dvi::Instruction;
use std::collections::HashMap;

// Treat the entire input as a single batch
pub fn text(dvi_bytes: &[u8]) -> Vec<u8> {
    let mut bytes_remaining = dvi_bytes;
    let mut machine = Machine::new();
    while bytes_remaining.len() > 0 {
        let (b, inst) =
            Instruction::parse(bytes_remaining).expect("Bytes should be a valid DVI file");
        machine.apply_inst(&inst);
        bytes_remaining = b;
    }
    machine.text
}

#[derive(Clone)]
struct Position {
    /// Horizontal coordinate for the current position in the page, in DVI units
    h: i32,
    /// Vertical coordinate for the current position in the page, in DVI units
    v: i32,
    /// Horizontal spacing coordinate *w*, in DVI units
    w: i32,
    /// Horizontal spacing coordinate *x*, in DVI units
    x: i32,
    /// Vertical spacing coordinate *y*, in DVI units
    y: i32,
    /// Vertical spacing coordinate *z*, in DVI units
    z: i32,
}

impl Position {
    pub fn zero() -> Position {
        Position {
            h: 0,
            v: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

struct CharPos {
    /// Horizontal coordinate for the character's position in the page, in DVI units
    h: i32,
    /// Vertical coordinate for the character's position in the page, in DVI units
    v: i32,
    /// Font index for the character
    font_index: u32,
    /// Char code of the character to draw
    code: u8,
}

struct Machine {
    position: Position,
    position_stack: Vec<Position>,
    font_index: u32,
    fonts: HashMap<u32, dvi::FontDef>,
    /// characters collected so far on the current page
    chars: Vec<CharPos>,
    text: Vec<u8>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            position: Position::zero(),
            position_stack: Vec::new(),
            font_index: 0,
            fonts: HashMap::new(),
            chars: Vec::new(),
            text: Vec::new(),
        }
    }
    fn put_char(&mut self, char: u8) {
        self.chars.push(CharPos {
            h: self.position.h,
            v: self.position.v,
            font_index: self.font_index,
            code: char,
        })
    }
    fn get_font(&self, font_index: u32) -> &dvi::FontDef {
        return self.fonts.get(&font_index).expect("Font should be defined");
    }
    fn char_width(&self, font_index: u32, _char: u8) -> i32 {
        // not using TFM files; just assume width is the design size.
        return self
            .get_font(font_index)
            .design_size
            .try_into()
            .expect("Design size should fit inside i32");
    }
    pub fn apply_inst(&mut self, inst: &Instruction) {
        use Instruction::*;
        match inst {
            Set(charcode) | Put(charcode) => {
                let char = u8::try_from(*charcode).expect("Char to set should fit in u8");
                self.put_char(char);
                if let Set(_) = inst {
                    self.position.h += self.char_width(self.font_index, char);
                }
            }
            PutRule(_, _) => {}
            SetRule(_, b) => {
                self.position.h += b;
            }
            Nop => {}
            Bop(_, _) => {
                self.font_index = 0;
                self.position = Position::zero();
                self.position_stack = Vec::new();
                // Currently just assuming the pages are in order in the DVI file; in particular, the ten
                // c_i parameters are unused, and the p parameter is unused
            }
            Eop => {
                assert!(
                    self.position_stack.len() == 0,
                    "Stack should be empty at end-of-page"
                );
                // Print what you have read since the previous bop
                // Sort by v coordinate first, then by h coordinate
                self.chars.sort_by(|a, b| (a.v, a.h).cmp(&(b.v, b.h)));
                // horizontal coordinate of the right edge of the previous character
                let mut prev_h = 0;
                // vertical coordinate of the previous character
                let mut prev_v = 0;
                for char_pos in &self.chars {
                    // insert newlines as necessary
                    // ignore the first newline of every page
                    let dy = char_pos.v - prev_v;
                    if dy > 0 && prev_v > 0 {
                        let design_size = self.get_font(char_pos.font_index).design_size;
                        // assume baseline skip averages 1.2*design_size
                        // division is equivalent to pseudocode:
                        // let baseline_skip = (design_size*1.2)
                        // let num_spaces = round(float_div(x / baseline_skip))
                        let num_newlines = ((dy as u32) * 5 + design_size / 2) / 6 / design_size;
                        for _ in 0..num_newlines {
                            self.text.push(b'\n');
                        }
                        prev_h = 0;
                    }
                    // insert spaces as necessary
                    let dx = char_pos.h - prev_h;
                    if dx > 0 {
                        let design_size = self.get_font(char_pos.font_index).design_size;
                        // assume width of a space averages (1/3)*design_size
                        // division is equivalent to pseudocode:
                        // let space_width = (design_size/3)
                        // let num_spaces = round(float_div(x / space_width))
                        let num_spaces = ((dx as u32) * 3 + design_size / 2) / design_size;
                        for _ in 0..num_spaces {
                            self.text.push(b' ');
                        }
                    }
                    // finally insert the character itself
                    self.text.push(char_pos.code);
                    // update coordinates
                    prev_v = char_pos.v;
                    prev_h = char_pos.h + self.char_width(char_pos.font_index, char_pos.code)
                }
                // Always trailing newline for each page
                self.text.push(b'\n');
                self.chars = Vec::new();
            }
            Push => self.position_stack.push(self.position.clone()),
            Pop => {
                self.position = self
                    .position_stack
                    .pop()
                    .expect("Stack should be non-empty on pop")
            }
            Right(b) => self.position.h += b,
            W(b) => {
                if let Some(w) = b {
                    self.position.w = *w;
                }
                self.position.h += self.position.w;
            }
            X(b) => {
                if let Some(x) = b {
                    self.position.x = *x;
                }
                self.position.h += self.position.x;
            }
            Down(b) => self.position.v += b,
            Y(b) => {
                if let Some(y) = b {
                    self.position.y = *y;
                }
                self.position.v += self.position.y;
            }
            Font(k) => self.font_index = *k,
            Xxx(_) => {}
            FontDef(font_def) => {
                self.fonts.insert(font_def.number, font_def.clone());
            }
            Z(b) => {
                if let Some(z) = b {
                    self.position.z = *z;
                }
                self.position.v += self.position.z;
            }
            Pre { .. } => {}
            Post { .. } => {}
            PostPost { .. } => {}
        }
    }
}
