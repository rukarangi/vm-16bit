use std::collections::HashMap;

fn create_memory(size: u16) -> Vec<u16> {
    let buffer: Vec<u16> = Vec::with_capacity(size as usize);
    return buffer;
}

struct Cpu {
    memory: Vec<u16>,
    register_names: Vec<String>,
    registers: Vec<u16>,
    register_map: HashMap<String, u16>
}

impl Cpu {
    fn new() -> Cpu {
        let memory = create_memory(u16::MAX);
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
        let registers = create_memory((register_names.len() * 2) as u16);

        let mut register_map = HashMap::new();
        for i in 0..register_names.len() {
            register_map.insert(
                &register_names[i],
                (i * 2) as u16
            );
        }

        return Cpu {
            memory,
            register_names,
            registers,
            register_map
        }
    }
}

fn main() {
    println!("Hello, world!");
}
