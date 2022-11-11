# DVI to Text

Converts a DVI file (as output from TeX/LaTeX) to a slice of bytes representing what might be the text displayed in the document. DVI files have no concept of Unicode; each character command draws a character (value 0 to 255) in a certain font, including fonts that might not have an ASCII subset. This tool discards the font information.

The tool inserts newline bytes `0x0A` for (some?) vertical gaps. Still ironing out details.

Includes a command-line interface to output to stdout. Example usage:

```
cargo run tests/abc.dvi
```
