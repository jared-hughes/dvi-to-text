# DVI to Text

Converts a DVI file (as output from TeX/LaTeX) to a slice of bytes representing what might be the text displayed in the document. DVI files have no concept of Unicode; each character command draws a character (value 0 to 255) in a certain font, including fonts that might not have an ASCII subset. This tool discards the font information.

The tool inserts newline bytes `0x0A` for vertical spaces and page breaks, and space bytes `0x20` for horizontal spaces between characters. Newline and space bytes are inserted proportional to the length of spaces.

Includes a command-line interface to output to stdout. Example usage:

```
dvi-to-text tests/abc.dvi
```
