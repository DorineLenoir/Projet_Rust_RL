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

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub des: u8,  // 2 bits
    pub op: u8,   // 2 bits
    pub src1: u8, // 3 bits
    pub src2: u8, // 3 bits
}

#[derive(Debug, PartialEq)]
pub struct Trame {
    pub instructions: [Instruction; 6],
} 

pub fn decode_trame(trame: u64) -> Trame {
    // Découpe la trame en six instructions de 10 bits, puis décode chacune.
    let instructions: [Instruction; 6] = std::array::from_fn(|i| {
        // Extrait les 10 bits de l'instruction située à l'indice `i`.
        let instruction_bits = ((trame >> (i * 10)) & 0x3FF) as u16;
        decode_instruction(instruction_bits)
    });

    // Assemble les instructions décodées dans une trame.
    Trame { instructions }
}
pub fn decode_instruction(instruction: u16) -> Instruction { 
    let des = ((instruction >> 8) & 0b11) as u8;
    let op = ((instruction >> 6) & 0b11) as u8;
    let src1 = ((instruction >> 3) & 0b111) as u8;
    let src2 = (instruction & 0b111) as u8;

    Instruction { des, op, src1, src2 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_instruction() {
        let instruction: u16 = 0b1101010011; // Exemple d'instruction
        let decoded = decode_instruction(instruction);
        assert_eq!(decoded.des, 0b11);
        assert_eq!(decoded.op, 0b01);
        assert_eq!(decoded.src1, 0b010);
        assert_eq!(decoded.src2, 0b011);
    }

    #[test]
    fn test_decode_trame() {
        // Les blocs sont écrits de l'instruction 5 à l'instruction 0 (la 0 est
        // dans les bits de poids faible, lus en premier par decode_trame).
        // J'ai rajouté les 4 bits qui doivent être ignorés pour voir si il le sont bien
        
        let trame: u64 = 0b1111_1111101101_1100011011_1011010110_0110101001_0001110010_0000000000;
        let trame2: u64 = 0b0000_1111101101_1100011011_1011010110_0110101001_0001110010_0000000000;

        let decoded_trame = decode_trame(trame);
        let decoded_trame2 = decode_trame(trame2);
        
        // Résultats attendus, dans l'ordre de lecture : des, op, src1, src2.
        let expected = [
            (0b00, 0b00, 0b000, 0b000), // instruction 0: 0000000000
            (0b00, 0b01, 0b110, 0b010), // instruction 1: 0001110010
            (0b01, 0b10, 0b101, 0b001), // instruction 2: 0110101001
            (0b10, 0b11, 0b010, 0b110), // instruction 3: 1011010110
            (0b11, 0b00, 0b011, 0b011), // instruction 4: 1100011011
            (0b11, 0b11, 0b101, 0b101), // instruction 5: 1111101101
        ];

        assert_eq!(decoded_trame, decoded_trame2);
    }    
}
