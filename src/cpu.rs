use crate::memory::Memory;
use prettytable::{Cell, Row, Table};

pub struct Cpu {
    memory: Memory,
    register_values: Vec<u16>,
    register_names: [String; 8],
}

impl Cpu {
    pub fn new(mem: Option<Memory>) -> Self {
        let mut register_values: Vec<u16> = Vec::new();
        let register_names = ["A", "B", "C", "D", "CA", "IR", "Z", "LR"].map(|x| x.to_owned());

        for _ in 0..16 {
            register_values.push(0b0000000000000000);
        }
        match mem {
            Some(memory) => Cpu {
                memory: memory,
                register_values,
                register_names,
            },
            _ => Cpu {
                memory: Memory::new(1024),
                register_values,
                register_names,
            },
        }
    }

    pub fn set_register(&mut self, name: &str, value: u16) -> bool {
        let position = self.register_names.iter().position(|x| x == name);
        match position {
            Some(index) => {
                self.register_values[index] = value;
                return true;
            }
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
            }
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
            }
            Err(x) => Err(x),
        }
    }

    fn print_instruction(&self, operand: u16, operator: u16, _type: &str) {
        println!("[CPU] {{DEBUG}} Instruction_type: {:?} Operand: {:#000000008b} Operator: {:#000000008b}", _type, operand, operator);
    }

    pub fn execute(&mut self, instruction: u16) {
        // divide the provided instruction in two sets operand and operator
        // the first 8 bits from the 16 bits are the operand and the other 8 bits are the operator(s)
        // eg: 0b1101 -> operand: 11 and operator: 01
        let operand = instruction >> 8;
        let operator = instruction & ((1 << 8) - 1);
        match operand {
            // Refactor this to to replace these magic numbers.

            /////////////////////////////////////////////////////////////
            //////////          Move Commands            ////////////////
            /////////////////////////////////////////////////////////////
            0b00000010 => {
                // Moving a literal value to register A
                self.print_instruction(operand, operator, "MOVA");
                self.set_register("A", operator);
            }
            0b00000011 => {
                // Moving a literal value to register B
                self.print_instruction(operand, operator, "MOVB");
                self.set_register("B", operator);
            }
            0b00000100 => {
                // Moving a literal value to register C
                self.print_instruction(operand, operator, "MOVC");
                self.set_register("C", operator);
            }
            0b00000101 => {
                // Moving a literal value to register D
                self.print_instruction(operand, operator, "MOVD");
                self.set_register("D", operator);
            }
            0b00000110 => {
                // Moving a value from register 1 to register 2
                self.print_instruction(operand, operator, "MOV A,B");
                let r1 = operator >> 4;
                let r2 = operator & ((1 << 4) - 1);
                let register_names = self.register_names.clone();
                let r1_name = register_names.get(r1 as usize);
                match r1_name {
                    Some(r1_name) => {
                        let r2_value =
                            self.get_register(self.register_names.get(r2 as usize).unwrap());
                        self.set_register(r1_name, r2_value.unwrap());
                    }
                    _ => todo!(),
                }
            }
            /////////////////////////////////////////////////////////////
            //////////          Arithemetic Commands     ////////////////
            /////////////////////////////////////////////////////////////
            0b00011010 => {
                // SUBA #
                self.print_instruction(operand, operator, "SUBA");
                let a_value = self.get_register("A").unwrap();
                self.set_register("A", a_value - operator);
            }
            0b00011011 => {
                // SUBA,B
                self.print_instruction(operand, operator, "SUB A,B");
                let r1 = operator >> 4;
                let r2 = operator & ((1 << 4) - 1);
                let register_names = self.register_names.clone();
                let r1_name = match register_names.get(r1 as usize) {
                    Some(r1_name_value) => r1_name_value,
                    _ => todo!(),
                };
                let r2_name = match register_names.get(r2 as usize) {
                    Some(r2_name_value) => r2_name_value,
                    _ => todo!(),
                };
                let r2_value = self.get_register(&r2_name);
                let r1_value = self.get_register(&r1_name);
                self.set_register(r1_name, r1_value.unwrap() - r2_value.unwrap());
            }
            _ => {
                println!(
                    "[CPU]: Instruction {:#16b} not implemented yet",
                    instruction
                );
            }
        }
    }

    pub fn debug(&self) {
        let mut table = Table::new();
        let mut headers: Vec<Cell> = Vec::new();
        let mut values: Vec<Cell> = Vec::new();

        self.register_names
            .iter()
            .enumerate()
            .for_each(|(_, x)| headers.push(Cell::new(&x)));

        self.register_values
            .iter()
            .enumerate()
            .for_each(|(_, x)| values.push(Cell::new(&(x.to_string()))));

        table.add_row(Row::new(headers));
        table.add_row(Row::new(values));
        table.printstd();
    }

    pub fn step(&mut self) {
        let ir_value = self.get_register("IR");
        match ir_value {
            Ok(data) => {
                let instruction = self.fetch_memory(data).unwrap();
                self.execute(instruction);
                self.set_register("IR", data + 1);
            }
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
                        self.debug();
                    } else {
                        break;
                    }
                }
                _ => println!("[CPU] Error getting IR value"),
            }
        }
    }
}
