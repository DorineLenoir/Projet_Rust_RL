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
use crate::decod::Instruction;


// Struct Interpreter : cela reprsénte l'état d'une execution d'un programme, avec les registres et l'entrée du programme
pub struct Interpreter{
    input:[f32; 4], // Entrée du programme x, x_dot, theta, theta_dot
    registres: [f32; 4], // Registres R0, R1, R2, R3
}


// Implémentation de l'interpréteur
impl Interpreter {

    // Crée un nouvel interpréteur avec l'entrée donnée
    // quand on a des entrées nouvelles, on crée un intepreteur avec ces entrées, et on execute le programme avec ces entrées
    // on initialise les registres à 0.0 car ils sont utilisés pour un seul programme
    pub fn new(input: [f32; 4]) -> Self {
        Interpreter {
            input,
            registres: [0.0; 4],
        }
    }

// Lit la valeur d'une source (src) donnée, qui peut être un registre ou une entrée
    pub fn read_src(&self,src:u8) -> f32 { // changer u64 par trame
        match src{
            0b000 => self.registres[0], // R0
            0b001 => self.registres[1], // R1
            0b010 => self.registres[2], // R2
            0b011 => self.registres[3], // R3
            0b100 => self.input[0],     // I0
            0b101 => self.input[1],     // I1
            0b110 => self.input[2],     // I2
            0b111 => self.input[3],     // I3
            _ => unreachable!("src must be 3 bits, got {src}"),
        }

    }

    // Écrit une valeur dans un registre donné (des)
    pub fn write_des(&mut self,des:u8,value:f32) {
        match des{
            0b00 => self.registres[0] = value, // R0
            0b01 => self.registres[1] = value, // R1
            0b10 => self.registres[2] = value, // R2
            0b11 => self.registres[3] = value, // R3
            _ => unreachable!("des must be 2 bits, got {des}"),
        }
    }

    // Effectue une opération binaire sur deux sources (src1 et src2) et retourne le résultat
    pub fn do_operation (&self,op:u8,src1:f32,src2:f32) -> f32 {
        let result = match op{
            0b00 => src1 + src2, // ADD
            0b01 => src1 - src2, // SUB
            0b10 => src1 * src2, // MUL
            0b11 => src1 / src2, // DIV
            _ => unreachable!("op must be 2 bits, got {op}"),
        };

        // au cas où c'est infini ou NaN, on ne veut pas que ça arrive, donc on panique
        if result.is_infinite() || result.is_nan() {
            unreachable!("Operation resulted in invalid value");
        }
        result
    }

    // Execute une séquence d'instructions sur l'interpréteur
    pub fn execute_decoded(&mut self, instruction: Instruction) {

            let src1= self.read_src(instruction.src1);
            let src2= self.read_src(instruction.src2);
            let result = self.do_operation(instruction.op, src1, src2);
            self.write_des(instruction.des, result);
        }

        // Exécute un programme donné (sous forme de tableau d'instructions) et retourne la valeur du registre R0 après l'exécution
      pub fn run(&mut self, program: u64) -> f32 {
            let instructions = decode_instruction(program);
            

            for instruction in instructions {
                self.execute_decoded(instruction);
            }

            self.registres[0]

        }
    }

