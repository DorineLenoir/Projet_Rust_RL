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

use crate::decod::{decode_instruction};

// Fonction Interpreteur
fn interpret_instruction(instruction: u64) {
    let decoded = decode_instruction(instruction);

    match decoded.des {
        0b00 => {
            // R0 = src1 op src2
        }
        0b01 => {
            // R1 = src1 op src2
        }
        0b10 => {
            // R2 = src1 op src2
        }
        0b11 => {
            // R3 = src1 op src2
        }
        _ => {}
    }

    match decoded.op {
        0b00 => {
            // ADD
        }
        0b01 => {
            // SUB
        }
        0b10 => {
            // MUL
        }
        0b11 => {
            // DIV
        }
        _ => {}
    }

    if decoded.src1 >= 0b000 && decoded.src1 <= 0b011 {
        // src1 is a register (R0, R1, R2, R3)
    } else if decoded.src1 >= 0b100 && decoded.src1 <= 0b111 {
        // src1 is (I0, I1, I2, I3)
    }

    if decoded.src2 >= 0b000 && decoded.src2 <= 0b011 {
        // src2 is a register (R0, R1, R2, R3)
    } else if decoded.src2 >= 0b100 && decoded.src2 <= 0b111 {
        // src2 is (I0, I1, I2, I3)
    }
}