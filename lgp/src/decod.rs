/// Des | OP | SRC1 | SRC2
/// 2b  | 2b | 3b   | 3b   = 10 bits/instruction
/// 00  | 00 | 000  | 000 R0
/// 01  | 01 | 001  | 001 R1
/// 10  | 10 | 010  | 010 R2
/// 11  | 11 | 011  | 011 R3
///     |    | 100  | 100 I0
///     |    | 101  | 101 I1
///     |    | 110  | 110 I2
///     |    | 111  | 111 I3

pub struct Instruction {
    pub des: u8,  // 2 bits
    pub op: u8,   // 2 bits
    pub src1: u8, // 3 bits
    pub src2: u8, // 3 bits
}

pub fn decode_instruction(instruction: u64) -> Instruction {
    let des = ((instruction >> 8) & 0b11) as u8;
    let op = ((instruction >> 6) & 0b11) as u8;
    let src1 = ((instruction >> 3) & 0b111) as u8;
    let src2 = (instruction & 0b111) as u8;

    Instruction { des, op, src1, src2 }
}