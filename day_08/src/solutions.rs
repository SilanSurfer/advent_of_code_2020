#[derive(Debug, Copy, Clone)]
enum InstructionName {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    name: InstructionName,
    val: i32,
    was_executed: bool,
}

impl Instruction {
    pub fn new(oper: &str) -> Self {
        let parsed: Vec<&str> = oper.split(' ').collect();
        let name = match parsed[0] {
            "acc" => InstructionName::Acc,
            "jmp" => InstructionName::Jmp,
            "nop" => InstructionName::Nop,
            _ => panic!("Unknown instruction!"),
        };
        let val: i32 = parsed[1].parse().expect("Couldn't convert to i32");
        Instruction {
            name,
            val,
            was_executed: false,
        }
    }
    pub fn mark_executed(&mut self) {
        self.was_executed = true;
    }
}

pub fn get_acc_value(input: &[&str]) -> i32 {
    let mut instructions = map_input_to_instructions(input);
    let mut acc: i32 = 0;
    let mut index: i32 = 0;
    loop {
        let instruction = &mut instructions[index as usize];
        println!("[{}] Executing {:?}", index, &instruction);
        if !instruction.was_executed {
            let (inc_acc, inc_index) = exectute_instruction(instruction);
            acc += inc_acc;
            index += inc_index;
        } else {
            break;
        }
    }
    acc
}

pub fn get_acc_value_when_no_loop(input: &[&str]) -> i32 {
    let instructions = map_input_to_instructions(input);
    for (no, elem) in instructions.iter().enumerate() {
        let mut index: i32 = 0;
        let mut acc: i32 = 0;
        let mut instructions_modified = instructions.clone();
        match elem.name {
            InstructionName::Acc => continue,
            InstructionName::Jmp => {
                instructions_modified[no].name = InstructionName::Nop;
                println!("Modified jmp at {}", no);
            }
            InstructionName::Nop => {
                instructions_modified[no].name = InstructionName::Jmp;
                println!("Modified nop at {}", no);
            }
        }
        loop {
            let instruction = &mut instructions_modified[index as usize];
            if !instruction.was_executed {
                let (inc_acc, inc_index) = exectute_instruction(instruction);
                acc += inc_acc;
                index += inc_index;
            } else {
                break;
            }
            if index as usize == instructions.len() {
                println!("Finished last instruction, current index: {}", index);
                return acc;
            }
        }
    }
    0
}

fn exectute_instruction(instruction: &mut Instruction) -> (i32, i32) {
    let mut acc: i32 = 0;
    let mut index: i32 = 0;
    match instruction.name {
        InstructionName::Acc => {
            acc += instruction.val;
            index += 1;
        }
        InstructionName::Jmp => {
            index += instruction.val;
        }
        InstructionName::Nop => {
            index += 1;
        }
    }
    instruction.mark_executed();
    (acc, index)
}

fn map_input_to_instructions(input: &[&str]) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| Instruction::new(line))
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod tests {
    const INPUT: [&str; 9] = [
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];
    #[test]
    fn test_first_part() {
        assert_eq!(5, super::get_acc_value(&INPUT));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(8, super::get_acc_value_when_no_loop(&INPUT));
    }
}
