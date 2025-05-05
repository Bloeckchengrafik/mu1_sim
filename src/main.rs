mod condition;
mod instruction;

use condition::Condition;
use instruction::{Instruction, Label};
use std::{collections::HashMap, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    let arg = args.get(1).expect("No argument provided");

    let file = std::fs::File::open(arg).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap();
    let lines = data.lines().map(|line| line.trim()).collect::<Vec<&str>>();
    let mut program = Vec::new();
    let mut labels = HashMap::new();
    let mut conditions = Vec::new();
    let mut monitoring_labels = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.contains(":") {
            let next_addr = program.len() as u16;
            labels.insert(line.strip_suffix(":").unwrap(), next_addr);
            continue;
        }

        if line.contains("!") {
            let condition = Condition::from(line.to_string());
            conditions.push(condition);
            continue;
        }

        if line.contains("%") {
            let label = line[1..].to_string();
            let monitoring_label = Label::Unresolved(label.clone());
            monitoring_labels.insert(label, monitoring_label);
            continue;
        }

        let instruction = Instruction::from(line.to_string());
        program.push(instruction);
    }

    program = program.iter().map(|inst| inst.resolve(&labels)).collect();

    conditions = conditions
        .iter()
        .map(|cond| cond.resolve(&labels))
        .collect();

    let monitoring_labels: HashMap<_, _> = monitoring_labels
        .iter()
        .map(|(label, monitoring_label)| (label.clone(), monitoring_label.resolve(&labels)))
        .collect();

    println!("Program: {:?}", program);
    println!("Conditions: {:?}", conditions);
    println!("Labels: {:?}", labels);

    let compiled_program: Vec<u16> = program.iter().map(|inst| inst.assemble()).collect();

    let mut acc = 0;
    let mut pc = 0u16;
    let mut sp = 255;
    let mut memory = vec![0u16; 256];
    for _ in 0..compiled_program.len() {
        memory[pc as usize] = compiled_program[pc as usize];
        pc += 1;
    }
    pc = 0;

    loop {
        let instruction = memory[pc as usize];
        pc += 1;

        let disasssembled = Instruction::from(instruction);
        println!("{:?} - PC: {}, AC: {}, SP: {}", disasssembled, pc, acc, sp);
        for (k, v) in &monitoring_labels {
            let address = v.get_address();
            println!("{}: {}", k, memory[address as usize]);
        }

        match disasssembled {
            Instruction::Lda(label) => {
                let address = label.get_address();
                acc = memory[address as usize];
            }
            Instruction::Sto(label) => {
                let address = label.get_address();
                memory[address as usize] = acc;
            }
            Instruction::Add(label) => {
                let address = label.get_address();
                acc = acc.wrapping_add(memory[address as usize]);
            }
            Instruction::Sub(label) => {
                let address = label.get_address();
                acc = acc.wrapping_sub(memory[address as usize]);
            }
            Instruction::Jmp(label) => {
                let address = label.get_address();
                pc = address;
            }
            Instruction::Jge(label) => {
                let address = label.get_address();
                if acc & 0b1000000000000000 == 0 {
                    pc = address;
                }
            }
            Instruction::Jne(label) => {
                let address = label.get_address();
                if acc != 0 {
                    pc = address;
                }
            }
            Instruction::Stop => break,
            Instruction::Call(label) => {
                sp -= 1;
                memory[sp as usize] = pc;
                let address = label.get_address();
                pc = address;
            }
            Instruction::Return => {
                pc = memory[sp as usize];
                sp += 1;
            }
            Instruction::Push => {
                sp -= 1;
                memory[sp as usize] = acc;
            }
            Instruction::Pop => {
                acc = memory[sp as usize];
                sp += 1;
            }
            Instruction::Ldr(label) => {
                let address = label.get_address();
                let next_address = memory[address as usize];
                acc = memory[next_address as usize];
            }
            Instruction::Str(label) => {
                let address = label.get_address();
                let next_address = memory[address as usize];
                memory[next_address as usize] = acc;
            }
            Instruction::MovPc => {
                pc = acc;
            }
            Instruction::MovSp => {
                sp = acc;
            }
            Instruction::Defw(_) => unreachable!(),
        }
    }

    for cond in conditions {
        cond.evaluate(&memory);
    }

    Ok(())
}
