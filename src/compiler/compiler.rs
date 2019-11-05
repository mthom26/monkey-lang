use crate::{
    compiler::{make_op, OpCode},
    evaluator::Object,
    lexer::lexer,
    parser::{parse, Expression, Operator, Prefix, Statement},
};

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
                    self.add_instruction(OpCode::OpPop);
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
            Expression::Boolean(val) => {
                match val {
                    true => self.add_instruction(OpCode::OpTrue),
                    false => self.add_instruction(OpCode::OpFalse),
                };
            }
            Expression::Infix { left, op, right } => {
                self.compile_expression(*left);
                self.compile_expression(*right);

                match op {
                    Operator::PLUS => self.add_instruction(OpCode::OpAdd),
                    Operator::MINUS => self.add_instruction(OpCode::OpSub),
                    Operator::MULTIPLY => self.add_instruction(OpCode::OpMul),
                    Operator::DIVIDE => self.add_instruction(OpCode::OpDiv),
                    Operator::GREATER => self.add_instruction(OpCode::OpGreater),
                    Operator::LESS => self.add_instruction(OpCode::OpLess),
                    Operator::EQUAL => self.add_instruction(OpCode::OpEqual),
                    Operator::NEQUAL => self.add_instruction(OpCode::OpNotEqual),
                };
            }
            Expression::Prefix { prefix, value } => {
                self.compile_expression(*value);

                match prefix {
                    Prefix::MINUS => self.add_instruction(OpCode::OpMinus),
                    Prefix::BANG => self.add_instruction(OpCode::OpBang),
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
            instructions: vec![1, 0, 0, 6],
            constants: vec![Object::Int(3)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 + 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 2, 6],
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
                6,       // OpPop
            ],
            constants: vec![Object::Int(1), Object::Int(2), Object::Int(3)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 - 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 3, 6],
            constants: vec![Object::Int(1), Object::Int(2)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 * 2 - 3 / 3 + 4";
        #[rustfmt::skip]
        let expected = ByteCode {
            instructions: vec![
                1, 0, 0, // Int 1
                1, 0, 1, // Int 2
                4,       // OpMul
                1, 0, 2, // Int 3
                1, 0, 3, // Int 3
                5,       // OpDiv
                3,       // OpSub
                1, 0, 4, // Int 4
                2,       // OpAdd
                6,       // OpPop
            ],
            constants: vec![
                Object::Int(1),
                Object::Int(2),
                Object::Int(3),
                Object::Int(3),
                Object::Int(4),
            ],
        };
        assert_eq!(expected, compiled(input));
    }

    #[test]
    fn test_booleans() {
        let input = "true";
        let expected = ByteCode {
            instructions: vec![7, 6],
            constants: vec![],
        };
        assert_eq!(expected, compiled(input));

        let input = "false;";
        let expected = ByteCode {
            instructions: vec![8, 6],
            constants: vec![],
        };
        assert_eq!(expected, compiled(input));
    }

    #[test]
    fn test_comparison_operators() {
        let input = "1 > 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 9, 6],
            constants: vec![Object::Int(1), Object::Int(2)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 < 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 10, 6],
            constants: vec![Object::Int(1), Object::Int(2)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 == 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 11, 6],
            constants: vec![Object::Int(1), Object::Int(2)],
        };
        assert_eq!(expected, compiled(input));

        let input = "1 != 2";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 1, 0, 1, 12, 6],
            constants: vec![Object::Int(1), Object::Int(2)],
        };
        assert_eq!(expected, compiled(input));
    }

    #[test]
    fn test_prefixes() {
        let input = "-1";
        let expected = ByteCode {
            instructions: vec![1, 0, 0, 14, 6],
            constants: vec![Object::Int(1)],
        };
        assert_eq!(expected, compiled(input));

        let input = "!false";
        let expected = ByteCode {
            instructions: vec![8, 13, 6],
            constants: vec![],
        };
        assert_eq!(expected, compiled(input));
    }
}
