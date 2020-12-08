use std::collections::HashSet;

use super::{ExecutionError, ExecutionResult, Instruction, Program};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ProgramState {
    pub instruction: usize,
    pub accumulator: i64,
}

impl ProgramState {
    fn modify(&self, p: &Program, offset: isize, acc: i64) -> ExecutionResult<ProgramState> {
        let new_instr = self.instruction as isize + offset;
        if new_instr < 0 || new_instr as usize > p.len() {
            Err(ExecutionError::InvalidAccess(new_instr as usize))
        } else {
            Ok(ProgramState {
                instruction: new_instr as usize,
                accumulator: self.accumulator + acc,
            })
        }
    }
}

pub struct VirtualMachine<'a> {
    program: &'a Program,
    current_state: ProgramState,
    visited: HashSet<usize>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(program: &'a Program) -> Self {
        VirtualMachine {
            program,
            current_state: ProgramState {
                instruction: 0,
                accumulator: 0,
            },
            visited: HashSet::new(),
        }
    }

    pub fn execute(&mut self) -> ExecutionResult<i64> {
        while !self.terminated() {
            self.execute_one()?;
        }
        Ok(self.current_state.accumulator)
    }

    pub fn execute_one(&mut self) -> ExecutionResult<()> {
        if self.terminated() {
            return Ok(());
        }

        let instr = self
            .program
            .get_instr(self.current_state.instruction)
            .ok_or(ExecutionError::InvalidAccess(
                self.current_state.instruction,
            ))?;
        let next_state = match instr {
            Instruction::Nop(_) => self.modify_state(1, 0)?,
            Instruction::Jmp(i) => self.modify_state(*i, 0)?,
            Instruction::Acc(i) => self.modify_state(1, *i)?,
        };
        if self.visited.insert(next_state.instruction) {
            self.current_state = next_state;
            Ok(())
        } else {
            Err(ExecutionError::InfiniteLoop(next_state.instruction))
        }
    }

    pub fn terminated(&self) -> bool {
        self.current_state.instruction == self.program.len()
    }

    pub fn current_state(&self) -> &ProgramState {
        &self.current_state
    }

    fn modify_state(&self, offset: isize, acc: i64) -> ExecutionResult<ProgramState> {
        self.current_state.modify(&self.program, offset, acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn runs_simple_program() {
        let p = Program::parse_lines(&["nop 1", "acc 2", "nop 2", "acc 2"]).unwrap();
        let mut vm = VirtualMachine::new(&p);
        assert_eq!(Ok(4), vm.execute());
    }
}
