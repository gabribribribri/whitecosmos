use std::io;

pub enum WSError {
    EOF,
}

type WSResult<T> = Result<T, WSError>;

pub struct WSInterpreter {
    code: String,
    reader: io::Lines<io::BufReader<std::fs::File>>,
    stack: Vec<i32>,
}

impl WSInterpreter {
    pub fn new(reader: io::Lines<io::BufReader<std::fs::File>>) -> Self {
        WSInterpreter {
            code: String::new(),
            reader,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> WSResult<()> {
        let mut code_index = 0;

        loop {
            match self.eval_statement() {
                Ok(index) => code_index = index,
                Err(WSError::EOF) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

    fn eval_statement(&mut self) -> WSResult<usize> {}

    // WARN This does not support UTF-8 at all. There are possibilities to do very very ugly things..........
    fn read_char(&mut self, index: usize) -> WSResult<u8> {
        while self.code.len() < index {
            let line = match self.reader.next() {
                Some(rs) => match rs {
                    Ok(l) => l,
                    Err(e) => todo!(),
                },
                None => return Err(WSError::EOF),
            };
            self.code.push_str(line.as_str());
        }
        Ok(self.code[index..index])
    }
}
