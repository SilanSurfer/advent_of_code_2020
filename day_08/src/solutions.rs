
#[derive(Debug)]
enum InstructionName {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
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
}

pub fn get_acc_value(input: &[&str]) -> i32 {
    let mut instructions: Vec<Instruction> = 
        input.iter().map(|line| Instruction::new(line)).collect();
    let mut acc: i32 = 0;
    let mut index: i32 = 0;
    loop {
        let mut instruction = &mut instructions[index as usize];
        println!("Executing instruction {:?}", &instruction);
        if !instruction.was_executed {
            match instruction.name {
                InstructionName::Acc => {
                    acc += instruction.val;
                    index += 1;
                    println!("Acc -> {}, next index -> {}", acc, index);
                }
                InstructionName::Jmp => {
                    index += instruction.val;
                    println!("Jmp -> {}", index);

                }
                InstructionName::Nop => {
                    index += 1;
                    println!("Nop -> {}", index);

                }
            }
            instruction.was_executed = true;
        } else {
            break;
        }
    }
    acc
}