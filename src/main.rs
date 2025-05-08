mod condition;
mod instruction;

use condition::Condition;
use instruction::{Instruction, Label};
use std::{collections::HashMap, io::Read};

fn transmute_to_signed(unsigned: u16) -> i16 {
    unsafe { std::mem::transmute(unsigned) }
}

fn query_value(
    memory: &Vec<u16>,
    symbols: &HashMap<&str, u16>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    println!("Enter memory address and format:");
    println!("Formats: i (signed), u (unsigned), x (hex), b (binary), n (instruction)");
    if !symbols.is_empty() {
        println!("Loaded syms! Use s to get symbols and use a symbol instead of an adress")
    }
    println!("Example: '42 i' or '100 x'");
    println!("Enter 'q' to quit");

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input == "q" {
            break;
        }

        if input == "s" {
            println!("Loaded symbols: ");
            for (symbol, addr) in symbols {
                println!("{}: {}", symbol, addr);
            }
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input format");
            continue;
        }

        let addr = if let Ok(direct_addr) = parts[0].parse::<usize>() {
            direct_addr
        } else if let Some(&symbol_addr) = symbols.get(parts[0]) {
            symbol_addr as usize
        } else {
            println!("Invalid address or unknown symbol");
            continue;
        };

        match parts[1] {
            "i" => println!(
                "Signed value at {}: {}",
                addr,
                transmute_to_signed(memory[addr])
            ),
            "u" => println!("Unsigned value at {}: {}", addr, memory[addr]),
            "x" => println!("Hex value at {}: 0x{:04X}", addr, memory[addr]),
            "b" => println!("Binary value at {}: {:016b}", addr, memory[addr]),
            "n" => match Instruction::try_from(memory[addr]) {
                Ok(inst) => println!("Instruction at {}: {:?}", addr, inst),
                Err(_) => println!("Invalid instruction at {}", addr),
            },
            _ => println!("Invalid format specifier (use 'i', 'u', 'x', 'b' or 'n')"),
        }
    }
    Ok(())
}

fn print_error(lines: &Vec<&str>, line: i32) {
    let red = "\x1b[31m"; // Red color
    let bold = "\x1b[1m"; // Bold text
    let reset = "\x1b[0m"; // Reset styling

    let start = if line > 2 { line - 2 } else { 0 } as usize;
    let end = (line + 2) as usize;
    for (i, code_line) in lines[start..end.min(lines.len())].iter().enumerate() {
        let current_line = start + i as usize;
        if current_line == (line as usize) {
            println!(
                "{}> {:3} | {}{}{}",
                red,
                current_line + 1,
                bold,
                code_line,
                reset
            );
            println!("      | {}{}{}", red, "^".repeat(code_line.len()), reset);
        } else {
            println!("  {:3} | {}", current_line + 1, code_line);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(arg) = args.get(1) else {
        println!("Usage: ./mu1_sim [filename]");
        return;
    };

    let Ok(file) = std::fs::File::open(arg) else {
        println!("File not found: {}", arg);
        return;
    };

    let mut reader = std::io::BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap();
    let lines = data.lines().map(|line| line.trim()).collect::<Vec<&str>>();
    let mut program = Vec::new();
    let mut labels = HashMap::new();
    let mut conditions = Vec::new();
    let mut monitoring_labels = HashMap::new();
    let mut line_index = -1;
    for mut line in lines.clone() {
        line_index += 1;
        line = line.split(";").next().unwrap();
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

        let Ok(instruction) = Instruction::try_from(line.to_string()) else {
            println!("Error while parsing instruction: Format not recognized");
            print_error(&lines, line_index);
            return;
        };
        program.push(instruction);
    }

    program = program
        .iter()
        .map(|inst| {
            let inst = inst.resolve(&labels);

            if let Err(val) = inst {
                panic!("Error while processing labels: {}", val);
            }
            inst.unwrap()
        })
        .collect();

    conditions = conditions
        .iter()
        .map(|cond| cond.resolve(&labels))
        .collect();

    let monitoring_labels: HashMap<_, _> = monitoring_labels
        .iter()
        .map(|(label, monitoring_label)| {
            let resolved = monitoring_label.resolve(&labels);

            if let Err(val) = resolved {
                panic!("Error while processing labels: {}", val);
            }

            return (label.clone(), resolved.unwrap());
        })
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

    query_value(&memory, &labels).unwrap();
}
