pub fn create_memory(size: u16) -> Vec<u8> {
    let buffer: Vec<u8> = vec![0u8; size as usize];
    return buffer;
}