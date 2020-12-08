use std::str::FromStr;

use super::{ParseError, ParseResult};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Nop(isize),
    Acc(i64),
    Jmp(isize),
}

impl Instruction {
    pub fn parse(s: &str) -> ParseResult<Instruction> {
        let s = s.trim();
        if s.len() == 0 {
            Err(ParseError::UnparseableLine(s.to_string()))
        } else {
            let instr_end = s.find(' ').unwrap_or(s.len());
            match &s[..instr_end] {
                x @ "nop" => Ok(Self::Nop(Self::parse_param(x, &s[instr_end..])?)),
                x @ "acc" => Ok(Self::Acc(Self::parse_param(x, &s[instr_end..])?)),
                x @ "jmp" => Ok(Self::Jmp(Self::parse_param(x, &s[instr_end..])?)),
                x => Err(ParseError::UnknownInstruction(x.to_string())),
            }
        }
    }

    fn parse_param<F: FromStr>(instr: &str, s: &str) -> ParseResult<F> {
        let s = s.trim();
        if s.len() == 0 {
            Err(ParseError::MissingParameter(instr.to_string()))
        } else {
            match s.parse() {
                Ok(i) => Ok(i),
                Err(_) => Err(ParseError::UnparseableParameter(
                    instr.to_string(),
                    s.to_string(),
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        assert_eq!(Ok(Instruction::Nop(-10)), Instruction::parse("nop -10"));
        assert_eq!(Ok(Instruction::Nop(10)), Instruction::parse("nop 10"));
        assert_eq!(Ok(Instruction::Acc(-10)), Instruction::parse("acc -10"));
        assert_eq!(Ok(Instruction::Acc(10)), Instruction::parse("acc 10"));
        assert_eq!(Ok(Instruction::Jmp(-10)), Instruction::parse("jmp -10"));
        assert_eq!(Ok(Instruction::Jmp(10)), Instruction::parse("jmp 10"));

        assert_eq!(
            Ok(Instruction::Nop(1)),
            Instruction::parse("    nop   1   ")
        );
    }

    #[test]
    fn parse_failure() {
        assert_eq!(
            Err(ParseError::UnparseableLine("".to_string())),
            Instruction::parse("")
        );

        assert_eq!(
            Err(ParseError::UnknownInstruction("p".to_string())),
            Instruction::parse("p")
        );
        assert_eq!(
            Err(ParseError::UnknownInstruction("p".to_string())),
            Instruction::parse("p")
        );
        assert_eq!(
            Err(ParseError::UnknownInstruction("pon".to_string())),
            Instruction::parse("pon")
        );
        assert_eq!(
            Err(ParseError::UnknownInstruction("pon".to_string())),
            Instruction::parse("pon 1")
        );

        assert_eq!(
            Err(ParseError::MissingParameter("nop".to_string())),
            Instruction::parse("nop")
        );

        assert_eq!(
            Err(ParseError::UnparseableParameter(
                "nop".to_string(),
                "a".to_string()
            )),
            Instruction::parse("nop a")
        );
    }
}
