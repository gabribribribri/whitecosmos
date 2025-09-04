use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    if let Ok(lines) = read_lines(&path) {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            println!("bam");
        }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
