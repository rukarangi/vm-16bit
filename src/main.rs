mod cpu;

const ACC: u8 = 0x01;
const R0: u8 = 0x02;
const R1: u8 = 0x03;
const R2: u8 = 0x04;

const FE: u8 = 0xFE;

fn main() {
    let mut mem1 = cpu::memory::create_memory(u16::MAX);

    let mut i: usize = 0;

    mem1[i] = cpu::instructions::MOV_MEM_REG; //13
    i += 1;
    mem1[i] = 0x01; // 0x0100 mem
    i += 1;
    mem1[i] = 0x00;
    i += 1;
    mem1[i] = R1;
    i += 1;

    mem1[i] = cpu::instructions::MOV_LIT_REG; //10
    i += 1;
    mem1[i] = 0x00; // 0x0001
    i += 1;
    mem1[i] = 0x01;
    i += 1;
    mem1[i] = R2;
    i += 1;

    mem1[i] = cpu::instructions::ADD_REG_REG; //14
    i += 1;
    mem1[i] = R1;
    i += 1;
    mem1[i] = R2;
    i += 1;

    mem1[i] = cpu::instructions::MOV_REG_MEM; //12
    i += 1;
    mem1[i] = ACC;
    i += 1;
    mem1[i] = 0x01; // 0x0100 mem
    i += 1;
    mem1[i] = 0x00;
    i += 1;

    mem1[i] = cpu::instructions::JNE_LIT_ADR; //15
    i += 1;
    mem1[i] = 0x00; // 0x0003
    i += 1;
    mem1[i] = 0x03;
    i += 1;
    mem1[i] = 0x00; // 0x0000 mem start
    i += 1;
    mem1[i] = 0x00;
    i += 1;
    mem1[i] = 0xFE;


    let mut cpu = cpu::Cpu::new(1 as u16);

    let length = mem1.len();

    cpu.insert(mem1);

    cpu.debug();
    cpu.view_mem_at(0x0100);

    for x in 0..length {
        if cpu.next_equals(0xFE) {
            break;
        }
        cpu.step();
        cpu.debug();
        let ip = match cpu.get_register(String::from("ip")) {
            Ok(x) => x,
            Err(e) => panic!("Error: {}", e)
        };
        cpu.view_mem_at(ip);
        cpu.view_mem_at(0x0100);
    }

    // cpu.step();
    // cpu.step();
    // cpu.step();
    
    // cpu.debug();

    // cpu.step();

    // cpu.debug();
    // cpu.view_mem_at(0x0100);
}
