fn main() {
    const MIN_U8: u8 = 0;
    const MAX_U8: u8 = 255;

    const MIN_I8: i8 = -128;
    const MAX_I8: i8 = 127;

    const HEX: u32 = 0xFF;
    const BINARY: u8 = 0b11111111;
    println!("{}", BINARY);

    let mult: u16 = (MAX_U8 * MAX_U8).into();
    println!("{}", mult);
}
