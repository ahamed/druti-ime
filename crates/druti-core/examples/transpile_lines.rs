//! Converts each line of standard input on its own, one output line per input
//! line. The Autocorrect list generator (`scripts/autocorrect/`) runs it to
//! see what the engine gives for a word (autocorrect design D3).

use std::io::{self, BufRead, BufWriter, Write};

use druti_core::transpile_roman_document;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for line in io::stdin().lock().lines() {
        writeln!(out, "{}", transpile_roman_document(&line?, false))?;
    }
    out.flush()
}
