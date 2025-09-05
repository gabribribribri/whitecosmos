mod ws_interpreter;

use std::fs::File;
use std::io;

use ws_interpreter::WSInterpreter;

fn main() -> io::Result<()> {
    let path: String = std::env::args().nth(1).unwrap();

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut wsi = WSInterpreter::new(reader);
    wsi.run().unwrap();
    Ok(())
}

// fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
