use crate::memory::Memory;

pub struct Cpu {
    memory: Memory,
    register_values: Vec<u16>,
    register_names: [String; 8]
}

impl Cpu {
    pub fn new(mem: Option<Memory>) -> Self {
        let mut register_values: Vec<u16> = Vec::new();
        let register_names = ["A", "B", "C", "D", "CA", "IR", "Z", "LR"].map(|x| x.to_owned());

        for _ in 0..16 {
            register_values.push(0b0000000000000000);
        }
        match mem {
            Some(memory) => {
                Cpu {
                    memory: memory,
                    register_values,
                    register_names
                }
            },
            _ => {
                Cpu {
                    memory: Memory::new(1024),
                    register_values,
                    register_names
                }

            }
        }
    }

    pub fn set_register(&mut self, name: &str, value: u16) -> bool {
        let position = self.register_names.iter().position(|x| x == name);
        match position {
            Some(index) => {
                self.register_values[index] = value;
                return true;
            },
            _ => {
                panic!("[CPU] NO SUCH REGISTER EXISTS");
            }
        }
    }

    pub fn get_register(&self, name: &str) -> Result<u16, &str> {
        let position = self.register_names.iter().position(|x| x == name);
        match position {
            Some(index) => {
                return Ok(self.register_values[index]);
            },
            _ => {
                return Err("[CPU]: No such register exists.");
            }
        }
    }

    fn fetch_memory(&self, position: u16) -> Result<u16, &str> {
        let memory_data = self.memory.get_data(position as usize);
        match memory_data {
            Ok(data) => {
                return Ok(data);
            },
            Err(x) => Err(x)
        }
    }

    pub fn execute(&mut self, instruction: u16) {
        // divide the provided instruction in two sets operand and operator
        // the first 8 bits from the 16 bits are the operand and the other 8 bits are the operator(s)
        // eg: 0b1101 -> operand: 11 and operator: 01
        let operand = instruction >> 8;
        let operator = instruction & ((1 << 8) - 1);
        println!("Operand: {:#8b} Operator: {:#8b}", operand, operator);
        match operand {
            // Refactor this to to replace these magic numbers.
            0b00000010 => {
                // Moving a literal value to register A
                self.set_register("A", operator);
            },
            _ => {
                println!("[CPU]: Instruction {:#16b} not implemented yet", instruction);
            }
        }
    }

    pub fn step(&mut self) {
        let ir_value = self.get_register("IR");
        match ir_value {
            Ok(data) => {
                let instruction = self.fetch_memory(data).unwrap();
                self.execute(instruction);
                self.set_register("IR", data + 1);
            },
            _ => {
                panic!("[CPU] IR VALUE not found.");
            }
        }
    }

    pub fn _loop(&mut self) {
        loop {
            match self.get_register("IR") {
                Ok(value) => {
                    if value >= 0 && (value as usize) < self.memory.len {
                        self.step();
                    } else {
                        break;
                    }
                },
                _ => println!("Loop ended.")

            }
        }
    }
}
