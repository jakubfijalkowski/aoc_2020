use std::io::{prelude::*, BufReader};
use std::{collections::HashSet, fs};
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Unknown instruction {0}")]
    UnknownInstruction(String),
    #[error("Missing parameter for instruction {0}")]
    MissingParameter(String),
    #[error("Unparseable parameter for instruction {0}")]
    UnparseableParameter(String),
    #[error("Cannot read input file")]
    IOError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl Instruction {
    fn parse(s: &str) -> Result<Instruction> {
        match &s[0..3] {
            x @ "nop" => Ok(Self::Nop(Self::parse_param(x, &s[4..])?)),
            x @ "acc" => Ok(Self::Acc(Self::parse_param(x, &s[4..])?)),
            x @ "jmp" => Ok(Self::Jmp(Self::parse_param(x, &s[4..])?)),
            x => Err(ParseError::UnknownInstruction(x.to_string())),
        }
    }

    fn parse_param(instr: &str, s: &str) -> Result<i64> {
        if s.len() == 0 {
            Err(ParseError::MissingParameter(instr.to_string()))
        } else {
            match s.trim().parse() {
                Ok(i) => Ok(i),
                Err(_) => Err(ParseError::UnparseableParameter(instr.to_string())),
            }
        }
    }

    fn switch(&self) -> Self {
        match self {
            Self::Nop(i) => Self::Jmp(*i),
            Self::Jmp(i) => Self::Nop(*i),
            x => *x,
        }
    }
}

struct Program {
    statements: Vec<Instruction>,
}

impl Program {
    fn from_file(path: &str) -> Result<Program> {
        let instr: Result<Vec<_>> = BufReader::new(fs::File::open(path)?)
            .lines()
            .map(|x| Instruction::parse(&x.unwrap()))
            .collect();
        Ok(Program { statements: instr? })
    }

    fn execute(&self, ip: usize, acc: i64) -> (usize, i64) {
        let instr = self.statements[ip];
        match instr {
            Instruction::Nop(_) => (ip + 1, acc),
            Instruction::Acc(i) => (ip + 1, acc + i),
            Instruction::Jmp(i) => (((ip as i64) + i) as usize, acc),
        }
    }
}

struct VM<'a> {
    program: &'a Program,
    visited: HashSet<usize>,
    acc: i64,
    ip: usize,
}

enum NextResult {
    Terminated(i64),
    InfiniteLoop(i64),
    Running,
}

enum ExecutionResult {
    Terminated(i64),
    InfiniteLoop(i64),
}

impl<'a> VM<'a> {
    fn new(program: &'a Program) -> Self {
        let mut visited = HashSet::new();
        visited.insert(0);
        VM {
            program,
            visited,
            acc: 0,
            ip: 0,
        }
    }

    fn execute(mut self) -> ExecutionResult {
        loop {
            match self.next() {
                NextResult::Running => {}
                NextResult::InfiniteLoop(i) => return ExecutionResult::InfiniteLoop(i),
                NextResult::Terminated(i) => return ExecutionResult::Terminated(i),
            }
        }
    }

    fn next(&mut self) -> NextResult {
        let (ip, acc) = self.program.execute(self.ip, self.acc);
        if ip == self.program.statements.len() {
            self.acc = acc;
            self.ip = ip;
            NextResult::Terminated(acc)
        } else if self.visited.insert(ip) {
            self.acc = acc;
            self.ip = ip;
            NextResult::Running
        } else {
            NextResult::InfiniteLoop(self.acc)
        }
    }
}

fn part1() -> Result<()> {
    let program = Program::from_file("./inputs/day08.txt")?;
    let vm = VM::new(&program);
    let result = match vm.execute() {
        ExecutionResult::InfiniteLoop(i) => i,
        _ => panic!("shouldn't terminate"),
    };
    println!("Part 1: {}", result);
    Ok(())
}

fn part2() -> Result<()> {
    let mut program = Program::from_file("./inputs/day08.txt")?;
    for i in 0..program.statements.len() {
        program.statements[i] = program.statements[i].switch();
        {
            let vm = VM::new(&program);
            let result = match vm.execute() {
                ExecutionResult::Terminated(i) => {
                    println!("Part 2: {}", i);
                    break;
                }
                _ => {}
            };
        }
        program.statements[i] = program.statements[i].switch();
    }

    Ok(())
}

fn main() {
    part1().unwrap();
    part2().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Instruction::Nop(-10),
            Instruction::parse("nop -10").unwrap()
        );
        assert_eq!(Instruction::Nop(10), Instruction::parse("nop 10").unwrap());
        assert_eq!(
            Instruction::Acc(-10),
            Instruction::parse("acc -10").unwrap()
        );
        assert_eq!(Instruction::Acc(10), Instruction::parse("acc 10").unwrap());
        assert_eq!(
            Instruction::Jmp(-10),
            Instruction::parse("jmp -10").unwrap()
        );
        assert_eq!(Instruction::Jmp(10), Instruction::parse("jmp 10").unwrap());
    }
}
