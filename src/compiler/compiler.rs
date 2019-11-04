use crate::compiler::{make_op, OpCode};
use crate::evaluator::Object;
use crate::lexer::lexer;
use crate::parser::{parse, Expression, Operator, Statement};

#[derive(Debug, PartialEq)]
pub struct ByteCode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}

impl ByteCode {
    fn new() -> Self {
        ByteCode {
            instructions: vec![],
            constants: vec![],
        }
    }
}

pub struct Compiler {
    byte_code: ByteCode,
}

impl Compiler {
    pub fn from_source(input: &str) -> ByteCode {
        let mut compiler = Compiler {
            byte_code: ByteCode::new(),
        };

        let mut tokens = lexer(input.as_bytes());
        let ast = parse(&mut tokens);

        compiler.compile_statements(ast);

        compiler.byte_code
    }

    fn compile_statements(&mut self, ast: Vec<Statement>) {
        for statement in ast {
            match statement {
                Statement::ExpressionStatement(expr) => {
                    self.compile_expression(expr);
                }
                _ => unimplemented!(),
            }
        }
    }

    fn compile_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Int(val) => {
                let index = self.add_constant(Object::Int(val));
                self.add_instruction(OpCode::OpConstant(index));
            }
            Expression::Infix { left, op, right } => {
                self.compile_expression(*left);
                self.compile_expression(*right);

                match op {
                    Operator::PLUS => self.add_instruction(OpCode::OpAdd),
                    _ => unimplemented!(),
                };
            }
            _ => unimplemented!(),
        }
    }

    // Add a value to the byte code constants and return the new index
    fn add_constant(&mut self, object: Object) -> u16 {
        self.byte_code.constants.push(object);
        (self.byte_code.constants.len() - 1) as u16
    }

    // Add instruction to byte code instructions and return instruction position
    fn add_instruction(&mut self, op_code: OpCode) -> u16 {
        let new_instruction_position = self.byte_code.instructions.len();
        let op_bytes = make_op(op_code);

        self.byte_code.instructions.extend(op_bytes);
        new_instruction_position as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::{ByteCode, Compiler},
        evaluator::Object,
    };

    fn compiled(input: &str) -> ByteCode {
        Compiler::from_source(input)
    }

    #[test]
    fn test_basic_expressions() {
        let input = "3";
        let expected = ByteCode {
            instructions: vec![1, 0, 0],
            constants: vec![Object::Int(3)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 + 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 2],
            constants: vec![Object::Int(1), Object::Int(2)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 + 2 + 3";
        #[rustfmt::skip]
        let expected = ByteCode {
            instructions: vec![
                1, 0, 0, // Int 1
                1, 0, 1, // Int 2
                2,       // OpAdd
                1, 0, 2, // Int 3
                2,       // OpAdd
            ],
            constants: vec![Object::Int(1), Object::Int(2), Object::Int(3)],
        };
        assert_eq!(expected, compiled(input));
    }
}
