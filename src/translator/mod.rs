pub struct Translator;

use crate::parser::Arithmetic;
use crate::parser::Operation;

impl Translator {
    pub fn translate(operations: &[Operation]) -> Vec<&str> {
        let mut instructions: Vec<&str> = Vec::new();

        for operation in operations {
            match operation {
                Operation::Arithmetic(arithmetic) => match arithmetic {
                    Arithmetic::Add => {
                        Translator::arithmetic_calculation_template(&mut instructions);
                        instructions.push(&"M=M+D");
                    }
                    Arithmetic::Sub => {
                        Translator::arithmetic_calculation_template(&mut instructions);
                        instructions.push(&"M=M-D");
                    }
                    Arithmetic::And => {
                        Translator::arithmetic_calculation_template(&mut instructions);
                        instructions.push(&"M=M&D");
                    }
                    Arithmetic::Or => {
                        Translator::arithmetic_calculation_template(&mut instructions);
                        instructions.push(&"M=M|D");
                    }
                    _ => panic!("Supported arithmetic operations: (add|sub|and|or)"),
                },
                _ => (),
            }
        }

        instructions
    }

    fn arithmetic_calculation_template(instructions: &mut Vec<&str>) {
        instructions.push(&"@SP");
        instructions.push(&"AM=M-1");
        instructions.push(&"D=M");
        instructions.push(&"A=A-1");
    }
}
