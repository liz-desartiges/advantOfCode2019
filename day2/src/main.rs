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
    fn new(slice: &[u32]) -> Result<Self, String> {
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

    fn run(self, program: &mut Vec<u32>) {
        let op1 = program[self.op1];
        let op2 = program[self.op2];
        match self.opcode {
            1 => program[self.dest] = op1 + op2,
            2 => program[self.dest] = op1 * op2,
            _ => unreachable!()
        }
    }
}

fn run(program: &mut Vec<u32>, noun: u8, verb: u8) -> u32 {
    let mut pos = 0;
    program[1] = noun as u32;
    program[2] = verb as u32;
    loop {
        match Instruction::new(&program[pos..pos+4]) {
            Ok(i) => i,
            Err(_s) => break,
        }.run(program);
        pos += 4;
    };
    program[0]
}


fn main() {
    let file = File::open("input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut program = String::new();
    buf_reader.read_to_string(&mut program).unwrap();
    program.pop();
    let program: Vec<u32> = program.split(",").map(|s| s.parse().unwrap()).collect();

    for verb in 0..100 {
        for noun in 0..100 {
            if run(&mut program.clone(), noun, verb) == 19690720 {
                println!("{}", 100u64 * noun as u64 + verb as u64);
            }
        }
    }
}
