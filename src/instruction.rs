use std::collections::HashMap;

#[derive(Debug)]
pub enum Label {
    Unresolved(String),
    Resolved(u16),
}

impl Label {
    pub fn resolve(&self, labels: &HashMap<&str, u16>) -> Self {
        match self {
            Label::Unresolved(name) => {
                if let Some(address) = labels.get(name.clone().to_string().as_str()) {
                    Label::Resolved(*address)
                } else {
                    panic!("Label {} not found", name);
                }
            }
            Label::Resolved(address) => Label::Resolved(*address),
        }
    }

    pub fn get_address(&self) -> u16 {
        match self {
            Label::Unresolved(_) => panic!("Label is unresolved"),
            Label::Resolved(address) => *address,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Lda(Label),
    Sto(Label),
    Add(Label),
    Sub(Label),
    Jmp(Label),
    Jge(Label),
    Jne(Label),
    Stop,
    Call(Label),
    Return,
    Push,
    Pop,
    Ldr(Label),
    Str(Label),
    MovPc,
    MovSp,
    Defw(Label),
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let opcode = value >> 12;
        let arg = value & 0b111111111111;
        match opcode {
            0 => Instruction::Lda(Label::Resolved(arg)),
            1 => Instruction::Sto(Label::Resolved(arg)),
            2 => Instruction::Add(Label::Resolved(arg)),
            3 => Instruction::Sub(Label::Resolved(arg)),
            4 => Instruction::Jmp(Label::Resolved(arg)),
            5 => Instruction::Jge(Label::Resolved(arg)),
            6 => Instruction::Jne(Label::Resolved(arg)),
            7 => Instruction::Stop,
            8 => Instruction::Call(Label::Resolved(arg)),
            9 => Instruction::Return,
            10 => Instruction::Push,
            11 => Instruction::Pop,
            12 => Instruction::Ldr(Label::Resolved(arg)),
            13 => Instruction::Str(Label::Resolved(arg)),
            14 => Instruction::MovPc,
            15 => Instruction::MovSp,
            _ => Instruction::Defw(Label::Resolved(arg)),
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() > 2 {
            panic!("Invalid instruction format: {}", value);
        }
        match parts[0].to_lowercase().as_str() {
            "lda" => Instruction::Lda(Label::Unresolved(parts[1].to_string())),
            "sto" => Instruction::Sto(Label::Unresolved(parts[1].to_string())),
            "add" => Instruction::Add(Label::Unresolved(parts[1].to_string())),
            "sub" => Instruction::Sub(Label::Unresolved(parts[1].to_string())),
            "jmp" => Instruction::Jmp(Label::Unresolved(parts[1].to_string())),
            "jge" => Instruction::Jge(Label::Unresolved(parts[1].to_string())),
            "jne" => Instruction::Jne(Label::Unresolved(parts[1].to_string())),
            "stp" => Instruction::Stop,
            "call" => Instruction::Call(Label::Unresolved(parts[1].to_string())),
            "return" => Instruction::Return,
            "push" => Instruction::Push,
            "pop" => Instruction::Pop,
            "ldr" => Instruction::Ldr(Label::Unresolved(parts[1].to_string())),
            "str" => Instruction::Str(Label::Unresolved(parts[1].to_string())),
            "movpc" => Instruction::MovPc,
            "movsp" => Instruction::MovSp,
            "defw" => {
                let as_u16: Option<u16> = parts[1].parse().ok();
                if as_u16.is_some() {
                    Instruction::Defw(Label::Resolved(as_u16.unwrap()))
                } else {
                    Instruction::Defw(Label::Unresolved(parts[1].to_string()))
                }
            }
            v => panic!("Unknown opcode {}", v),
        }
    }
}

impl Instruction {
    pub fn resolve(&self, labels: &HashMap<&str, u16>) -> Self {
        match self {
            Instruction::Lda(label) => Instruction::Lda(label.resolve(labels)),
            Instruction::Sto(label) => Instruction::Sto(label.resolve(labels)),
            Instruction::Add(label) => Instruction::Add(label.resolve(labels)),
            Instruction::Sub(label) => Instruction::Sub(label.resolve(labels)),
            Instruction::Jmp(label) => Instruction::Jmp(label.resolve(labels)),
            Instruction::Jge(label) => Instruction::Jge(label.resolve(labels)),
            Instruction::Jne(label) => Instruction::Jne(label.resolve(labels)),
            Instruction::Stop => Instruction::Stop,
            Instruction::Call(label) => Instruction::Call(label.resolve(labels)),
            Instruction::Return => Instruction::Return,
            Instruction::Push => Instruction::Push,
            Instruction::Pop => Instruction::Pop,
            Instruction::Ldr(label) => Instruction::Ldr(label.resolve(labels)),
            Instruction::Str(label) => Instruction::Str(label.resolve(labels)),
            Instruction::MovPc => Instruction::MovPc,
            Instruction::MovSp => Instruction::MovSp,
            Instruction::Defw(label) => Instruction::Defw(label.resolve(labels)),
        }
    }

    pub fn assemble(&self) -> u16 {
        let right_side = match &self {
            Instruction::Lda(addr) => addr.get_address(),
            Instruction::Sto(addr) => addr.get_address(),
            Instruction::Add(addr) => addr.get_address(),
            Instruction::Sub(addr) => addr.get_address(),
            Instruction::Jmp(addr) => addr.get_address(),
            Instruction::Jge(addr) => addr.get_address(),
            Instruction::Jne(addr) => addr.get_address(),
            Instruction::Stop => 0,
            Instruction::Call(addr) => addr.get_address(),
            Instruction::Return => 0,
            Instruction::Push => 0,
            Instruction::Pop => 0,
            Instruction::Ldr(addr) => addr.get_address(),
            Instruction::Str(addr) => addr.get_address(),
            Instruction::MovPc => 0,
            Instruction::MovSp => 0,
            Instruction::Defw(label) => label.get_address(),
        };

        let left_side = match self {
            Instruction::Lda(_) => 0,
            Instruction::Sto(_) => 1,
            Instruction::Add(_) => 2,
            Instruction::Sub(_) => 3,
            Instruction::Jmp(_) => 4,
            Instruction::Jge(_) => 5,
            Instruction::Jne(_) => 6,
            Instruction::Stop => 7,
            Instruction::Call(_) => 8,
            Instruction::Return => 9,
            Instruction::Push => 10,
            Instruction::Pop => 11,
            Instruction::Ldr(_) => 12,
            Instruction::Str(_) => 13,
            Instruction::MovPc => 14,
            Instruction::MovSp => 15,
            Instruction::Defw(_) => 0,
        };

        // 4 bit instruction, 12 bits address
        return (left_side << 12) | right_side;
    }
}
