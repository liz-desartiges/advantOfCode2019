use std::io::BufReader;
use std::fs::File;
use std::io::Read;

struct Instruction {
    opcode: u8,
    op1: usize,
    op2: usize,
    dest: usize
}

impl Instruction {
    fn new(slice: &[u64]) -> Result<Self, String> {
        match slice[0] {
            99 =>  Err("Program Halted!".to_string()),
            1 | 2 => {
                Ok(Self {
                    opcode: slice[0] as u8,
                    op1: slice[1] as usize,
                    op2: slice[2] as usize,
                    dest: slice[3] as usize,
                })
            },
            _ => Err("Unknown Op code".to_string()),
        }
    }

    fn run(self, program: &mut Vec<u64>) {
        let op1 = program[self.op1];
        let op2 = program[self.op2];
        match self.opcode {
            1 => program[self.dest] = op1 + op2,
            2 => program[self.dest] = op1 * op2,
            _ => unreachable!()
        }
    }
}


fn main() {
    let file = File::open("input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut program = String::new();
    buf_reader.read_to_string(&mut program).unwrap();
    program.pop();
    let mut program: Vec<u64> = program.split(",").map(|s| s.parse().unwrap()).collect();
    let mut pos = 0;
    let r = loop {
        match Instruction::new(&program[pos..pos+4]) {
            Ok(i) => i,
            Err(s) => break format!("{}", s),
        }.run(&mut program);
        pos += 4;
    };
    let program: Vec<String> = program.iter().map(|s| s.to_string()).collect();
    println!("{}", program.join(","));
    println!("{}", r);
}
