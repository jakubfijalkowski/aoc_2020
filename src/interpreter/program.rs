use std::{fs, io, io::BufRead, path::Path};

use super::{CodeParseError, CodeParseResult, Instruction};

pub struct Program {
    statements: Vec<Instruction>,
}

impl Program {
    pub fn parse_lines<'a>(lines: &'a [&'a str]) -> CodeParseResult<Self> {
        let i = lines.iter().map(|x| *x);
        Self::parse_iter(i)
    }

    pub fn parse_file<P: AsRef<Path> + Copy>(path: P) -> CodeParseResult<Self> {
        let lines: io::Result<Vec<String>> =
            fs::File::open(path).and_then(|file| io::BufReader::new(file).lines().collect());
        let lines = lines.map_err(|e| CodeParseError::from_io(path, e))?;
        let lines = lines.iter().map(String::as_ref);
        Ok(Self::parse_iter(lines)?)
    }

    fn parse_iter<'a, I: Iterator<Item = &'a str>>(iter: I) -> CodeParseResult<Self> {
        let statements: CodeParseResult<Vec<_>> = iter
            .enumerate()
            .map(|(i, l)| Instruction::parse(l).map_err(|e| CodeParseError::at_line(i, e)))
            .collect();
        let statements = statements?;
        Ok(Program { statements })
    }

    pub fn get_instr(&self, at: usize) -> Option<&Instruction> {
        self.statements.get(at)
    }

    pub fn set_instr(&mut self, at: usize, i: Instruction) {
        self.statements[at] = i;
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        let p = Program::parse_lines(&["nop 1", "jmp 2"]).unwrap();
        assert_eq!(vec![Instruction::Nop(1), Instruction::Jmp(2)], p.statements);
    }

    #[test]
    fn parse_failure() {
        let p = Program::parse_lines(&["nop 1", "err"]);
        assert!(matches!(p, Err(CodeParseError::AtLine { line: 1, .. })));

        let p = Program::parse_file("this file does not exist.txt");
        assert!(matches!(p, Err(CodeParseError::IOError { .. })))
    }
}
