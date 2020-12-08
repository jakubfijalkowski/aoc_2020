use aoc_2020::interpreter::*;
use std::io::{prelude::*, BufReader};
use std::{collections::HashSet, fs};
use thiserror::Error;

fn mutate(i: &Instruction) -> Instruction {
    match i {
        Instruction::Nop(i) => Instruction::Jmp(*i),
        Instruction::Jmp(i) => Instruction::Nop(*i),
        x => *x,
    }
}

fn part1() {
    let program = Program::parse_file("./inputs/day08.txt").unwrap();
    let mut vm = VirtualMachine::new(&program);
    let result = match vm.execute() {
        Err(ExecutionError::InfiniteLoop(_)) => vm.current_state().accumulator,
        _ => panic!("not looped!"),
    };
    println!("Part 1: {}", result);
}

fn part2() {
    let mut program = Program::parse_file("./inputs/day08.txt").unwrap();
    for i in 0..program.len() {
        program.set_instr(i, mutate(program.get_instr(i).unwrap()));
        {
            let mut vm = VirtualMachine::new(&program);
            match vm.execute() {
                Ok(i) => {
                    println!("Part 2: {}", i);
                    break;
                }
                _ => {}
            };
        }
        program.set_instr(i, mutate(program.get_instr(i).unwrap()));
    }
}

fn main() {
    part1();
    part2();
}
