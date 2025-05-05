use std::collections::HashMap;

use crate::instruction::Label;

#[derive(Debug)]
pub enum Condition {
    Equals(Label, u16),
}

impl From<String> for Condition {
    fn from(value: String) -> Self {
        if !value.starts_with("!") {
            panic!("invalid format: {}", value)
        }

        let args = value.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 2 {
            panic!("invalid format: {}", value)
        }

        let fnselect = args[0].to_string();

        match fnselect.as_str() {
            "!eq" => {
                let label = Label::Unresolved(args[1].to_string());
                let value = args[2].parse().unwrap();
                Condition::Equals(label, value)
            }
            _ => panic!("invalid format: {}", value),
        }
    }
}

impl Condition {
    pub fn resolve(&self, labels: &HashMap<&str, u16>) -> Self {
        match self {
            Condition::Equals(label, value) => {
                let resolved_label = label.resolve(labels);
                Condition::Equals(resolved_label, *value)
            }
        }
    }

    pub fn evaluate(&self, mem: &[u16]) {
        match self {
            Condition::Equals(label, value) => {
                let resolved_label = label.get_address();
                if mem[resolved_label as usize] == *value {
                    println!("Condition met! Value: {}", mem[resolved_label as usize]);
                } else {
                    println!("Condition not met! Value: {}", mem[resolved_label as usize]);
                }
            }
        }
    }
}
