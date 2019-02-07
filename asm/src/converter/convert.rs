use log::{info, error};

use instruction::Instruction;
use compiler::miscellaneous;
use compiler::control;
use compiler::arithmetic;

// converts instructions in complex assembly to base assembly

pub fn convert(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    for instruction in instructions {
        compiled.append(&mut convert_instruction(instruction));
    }

    compiled
}

pub fn convert_instruction(instruction: Instruction) -> Vec<Instruction> {
    info!("Instruction: {}", instruction.to_string());

    match instruction.command.as_str() {
        "MOVE" => {
            Vec::new()
        },
        "PUSH" => {
            Vec::new()
        },
        "POP" => {
            Vec::new()
        },
        "ADD" => {
            Vec::new()
        },
        "SUB" => {
            Vec::new()
        },
        "AND" => {
            Vec::new()
        },
        "OR" => {
            Vec::new()
        },
        "NOT" => {
            vec!(instruction)
        },
        "SHIFT_LEFT" => {
            vec!(instruction)
        },
        "SHIFT_LEFT_W" => {
            vec!(instruction)
        },
        "SHIFT_RIGHT" => {
            vec!(instruction)
        },
        "SHIFT_RIGHT_W" => {
            vec!(instruction)
        },
        "JUMP" => {
            vec!(instruction)
        },
        "JUMP0" => {
            vec!(instruction)
        },
        "NOP" => {
            vec!(instruction)
        },
        _ => {
            error!("Instruction not handled!");
            Vec::new()
        }
    }
}
