use std::collections::HashMap;

pub mod instructions;
pub mod memory;

pub struct Cpu {
    memory: Vec<u8>,
    register_names: Vec<String>,
    registers: Vec<u8>,
    register_map: HashMap<String, u8>
}

impl Cpu {
    pub fn debug(&self) {
        for name in self.register_names.iter() {
            let content = match self.get_register(name.to_string()) {
                Ok(x) => x,
                Err(e) => panic!("Error: {}", e)
            };

            println!("{:>4}: 0x{:0>4}", name, format!("{:X?}",content));
        }
        println!("");
    }

    pub fn view_mem_at(&self, address: u16) {
        let next_eight = (self.memory[(address as usize)..(address as usize + 8)]).to_vec();
        print!("0x{:0>4}:", format!("{:X?}",address));
        for x in next_eight.iter() {    
            print!(" 0x{:0>2} ", format!("{:X?}",x));
        }
        println!("\n");
    }

    pub fn next_equals(&self, value: u8) -> bool {
        let index = match self.get_register(String::from("ip")) {
            Ok(x) => x,
            Err(e) => panic!("Error: {}", e)
        };
        let next = self.memory[index as usize];
        return next == value;
    }

    pub fn insert(&mut self, memory: Vec<u8>) {
        self.memory = memory;
    }

    pub fn new(size: u16) -> Cpu {
        let memory = memory::create_memory(size);
        let register_names = vec![
            String::from("ip"),
            String::from("acc"),
            String::from("r0"),
            String::from("r1"),
            String::from("r2"),
            String::from("r3"),
            String::from("r4"),
            String::from("r5"),
            String::from("r6"),
            String::from("r7")
        ];
        let registers = memory::create_memory((register_names.len() * 2) as u16);

        let reg_nam = [
            "ip","acc",
            "r0","r1","r2","r3","r4","r5","r6","r7"
        ];

        let mut register_map = HashMap::new();
        for i in 0..register_names.len() {
            register_map.insert(
                reg_nam[i].to_string(),
                (i * 2) as u8
            );
        }

        return Cpu {
            memory,
            register_names,
            registers,
            register_map
        }
    }

    pub fn get_register(&self, name: String) -> Result<u16, String> {
        if !self.register_names.contains(&name) {
            return Err(format!("No register: {}", name))
        }   

        let idx = match self.register_map.get(&name) {
            Some(x) => x,
            None => &(0 as u8)
        };

        let top = ((self.registers[*idx as usize] as u16) << 8) as u16;
        let bottom = (self.registers[(*idx + 1) as usize]) as u16;
        let result = top | bottom;

        return Ok(result);
    }

    // pub fn find_register(&self, index: u8) -> Option<String> {
    //     return *self.register_map.iter()
    //         .find_map(|(key, &val)| if val == index { Some(key )} else { None });
    

    // }

    pub fn set_register(&mut self, name: &String, value: u16) -> Result<(), String> {
        if !self.register_names.contains(name) {
            return Err(format!("No register: {}", name));
        }

        let idx = match self.register_map.get(name) {
            Some(x) => x,
            None => &(0 as u8)
        };

        let top = (value >> 8) as u8;
        let bottom = value as u8;

        self.registers[*idx as usize] = top;
        self.registers[(*idx+1) as usize] = bottom;

        return Ok(());
    }

    pub fn fetch(&mut self) -> u8 {
        let next_addr = match self.get_register(String::from("ip")) {
            Ok(y) => y,
            Err(e) => panic!("Error: {}", e)
        };

        let instruction = self.memory[next_addr as usize];

        match self.set_register(&String::from("ip"), next_addr + 0x0001) {
            Ok(_) => return instruction,
            Err(e) => panic!("Error: {}", e)
        }
    }
    
    pub fn fetch_16(&mut self) -> u16 {
        let next_addr = match self.get_register(String::from("ip")) {
            Ok(y) => y,
            Err(e) => panic!("Error: {}", e)
        };

        let instruction_top = ((self.memory[next_addr as usize] as u16) << 8) as u16;
        let instruction_bottom = self.memory[next_addr as usize + 1] as u16;
        let instruction: u16 = instruction_top | instruction_bottom;

        match self.set_register(&String::from("ip"), next_addr + 0x0002) {
            Ok(_) => return instruction,
            Err(e) => panic!("Error: {}", e)
        }
    }

    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            // Move literal into register
            instructions::MOV_LIT_REG => {
                let literal: u16 = self.fetch_16();
                let register_idx: u8 = self.fetch();

                let register_name = &self.register_names[register_idx as usize].clone();

                match self.set_register(register_name, literal) {
                    Ok(_) => (),
                    Err(e) => panic!("Error: {}", e)
                }
            },
            // Move register into register
            instructions::MOV_REG_REG => {
                let r1 = self.fetch();
                let r1_value_top = ((self.registers[r1 as usize * 2] as u16) << 8) as u16;
                let r1_value_bottom = self.registers[r1 as usize * 2 + 1] as u16;
                let r1_value = r1_value_top | r1_value_bottom;

                let r2: u8 = self.fetch();
                let register_name = &self.register_names[r2 as usize].clone();

                match self.set_register(register_name, r1_value) {
                    Ok(_) => (),
                    Err(e) => panic!("Error: {}", e)
                }
            },
            // Move register to memory
            instructions::MOV_REG_MEM => {
                let r1: u8 = self.fetch();
                let r1_value_top = self.registers[r1 as usize * 2] as u8;
                let r1_value_bottom = self.registers[r1 as usize * 2 + 1] as u8;

                let address: u16 = self.fetch_16();
                // let top = (address >> 8) as u8;
                // let bottom = address as u8;

                self.memory[address as usize] = r1_value_top;
                self.memory[address as usize + 1] = r1_value_bottom;
            },
            // Move memory to register
            instructions::MOV_MEM_REG => {
                let mem_addr: u16 = self.fetch_16();
                let mem_top = (self.memory[mem_addr as usize] as u16) << 8;
                let mem_bottom = self.memory[mem_addr as usize + 1] as u16;
                let mem_value = mem_top | mem_bottom;

                let r1: u8 = self.fetch();
                let register_name = &self.register_names[r1 as usize].clone();

                match self.set_register(register_name, mem_value) {
                    Ok(_) => (),
                    Err(e) => panic!("Error: {}", e)
                }
            },
            // Add register to register
            instructions::ADD_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();

                let r1_value_top = ((self.registers[r1 as usize * 2] as u16) << 8) as u16;
                let r1_value_bottom = self.registers[r1 as usize * 2 + 1] as u16;
                let r1_value = r1_value_top | r1_value_bottom;

                let r2_value_top = ((self.registers[r2 as usize * 2] as u16) << 8) as u16;
                let r2_value_bottom = self.registers[r2 as usize * 2 + 1] as u16;
                let r2_value = r2_value_top | r2_value_bottom;

                //println!("{:X?} + {:X?}", r1_value_top, r1_value_bottom);
                
                match self.set_register(&String::from("acc"), r1_value+r2_value) {
                    Ok(_) => (),
                    Err(e) => panic!("Error: {}", e)
                }
            },
            // Jump if not equal
            instructions::JNE_LIT_ADR => {
                let value = self.fetch_16();
                let address = self.fetch_16();

                let acc = match self.get_register(String::from("acc")) {
                    Ok(y) => y,
                    Err(e) => panic!("Error: {}", e)
                };

                if value != acc {
                    match self.set_register(&String::from("ip"), address) {
                        Ok(_) => (),
                        Err(e) => panic!("Error: {}", e)
                    }
                }
            },
            
            _ => ()

        }

    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }
}

