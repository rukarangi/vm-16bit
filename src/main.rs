mod cpu;

fn main() {
    let mut mem1 = cpu::memory::create_memory(u8::MAX);

    mem1[0] = cpu::instructions::MOV_LIT_R1;
    mem1[1] = 0x12; // 0x1234
    mem1[2] = 0x34;

    mem1[3] = cpu::instructions::MOV_LIT_R2;
    mem1[4] = 0xAB; // 0xABCD
    mem1[5] = 0xCD;

    mem1[6] = cpu::instructions::ADD_REG_REG;
    mem1[7] = 0x03; // r1 index
    mem1[8] = 0x04; // r2 index

    let mut cpu = cpu::Cpu::new(1 as u8);

    cpu.insert(mem1);

    cpu.step();
    cpu.step();
    cpu.step();
    
    cpu.debug();
}
