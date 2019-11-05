use crate::{
    compiler::{two_u8_to_usize, ByteCode},
    evaluator::Object,
};
use std::mem;

const STACK_SIZE: usize = 2048;

pub struct Vm {
    instructions: Vec<u8>,
    constants: Vec<Object>,
    stack: [Object; STACK_SIZE],
    stack_pointer: usize,
}

impl Vm {
    fn new(bytecode: ByteCode) -> Self {
        Vm {
            instructions: bytecode.instructions,
            constants: bytecode.constants,
            stack: unsafe { mem::zeroed() },
            stack_pointer: 0,
        }
    }

    fn run(&mut self) {
        let mut ip = 0;

        while ip < self.instructions.len() {
            match self.instructions[ip] {
                0x01 => {
                    // OpConstant
                    let const_index =
                        two_u8_to_usize(self.instructions[ip + 1], self.instructions[ip + 2]);

                    self.push(self.constants[const_index].clone());
                    ip += 3;
                }
                0x02 => {
                    // OpAdd
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Int(left + right));
                        }
                        _ => panic!("Invalid OpAdd operand"),
                    };
                    ip += 1;
                }
                0x03 => {
                    // OpSub
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Int(left - right));
                        }
                        _ => panic!("Invalid OpSub operand"),
                    };
                    ip += 1;
                }
                0x04 => {
                    // OpMul
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Int(left * right));
                        }
                        _ => panic!("Invalid OpMul operand"),
                    };
                    ip += 1;
                }
                0x05 => {
                    // OpDiv
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            // TODO Handle remainders, currently they are truncated
                            self.push(Object::Int(left / right));
                        }
                        _ => panic!("Invalid OpDiv operand"),
                    };
                    ip += 1;
                }
                0x06 => {
                    // OpPop
                    self.pop();
                    ip += 1;
                }
                0x07 => {
                    // OpTrue
                    self.push(Object::Boolean(true));
                    ip += 1;
                }
                0x08 => {
                    // OpFalse
                    self.push(Object::Boolean(false));
                    ip += 1;
                }
                0x09 => {
                    // OpGreater
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Boolean(left > right));
                        }
                        _ => panic!("Invalid OpGreater operand"),
                    };
                    ip += 1;
                }
                0x0a => {
                    // OpLess
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Boolean(left < right));
                        }
                        _ => panic!("Invalid OpLess operand"),
                    };
                    ip += 1;
                }
                0x0b => {
                    // OpEqual
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Boolean(left == right));
                        }
                        _ => panic!("Invalid OpEqual operand"),
                    };
                    ip += 1;
                }
                0x0c => {
                    // OpNotEqual
                    match (self.pop(), self.pop()) {
                        (Object::Int(right), Object::Int(left)) => {
                            self.push(Object::Boolean(left != right));
                        }
                        _ => panic!("Invalid OpNotEqual operand"),
                    };
                    ip += 1;
                }
                0x0d => {
                    // OpBang
                    match self.pop() {
                        Object::Boolean(val) => self.push(Object::Boolean(!val)),
                        _ => panic!("Invalid OpBang operand"),
                    };
                    ip += 1;
                }
                0x0e => {
                    // OpMinus
                    match self.pop() {
                        Object::Int(val) => self.push(Object::Int(-val)),
                        _ => panic!("Invalid OpMinus operand"),
                    };
                    ip += 1;
                }
                invalid => panic!("Invalid instruction: {}", invalid),
            }
        }
    }

    fn push(&mut self, obj: Object) {
        if self.stack_pointer >= STACK_SIZE {
            panic!("Stack overflow");
        }

        self.stack[self.stack_pointer] = obj;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> Object {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer].clone()
    }

    // Utility function to observe stack
    fn print_stack(&self, num: usize) {
        println!("Vm Stack");
        for i in 0..num {
            println!("{}: {:?}", i, self.stack[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::{ByteCode, Compiler},
        evaluator::Object,
        vm::Vm,
    };

    fn compiled(input: &str) -> ByteCode {
        Compiler::from_source(input)
    }

    #[test]
    fn test_basics() {
        let input = "7";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Int(7), vm.stack[0]);

        let input = "1 + 2";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Int(3), vm.stack[0]);

        let input = "2 * 3";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Int(6), vm.stack[0]);

        let input = "2 * 2 + 6 / 2 - 9";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Int(-2), vm.stack[0]);

        let input = "1; 2; 3;";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Int(3), vm.stack[0]);

        let input = "false";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(false), vm.stack[0]);

        let input = "true;";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(true), vm.stack[0]);
    }

    #[test]
    fn test_comparisons() {
        let input = "1 < 2";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(true), vm.stack[0]);

        let input = "1 > 2";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(false), vm.stack[0]);

        let input = "3 == 3";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(true), vm.stack[0]);

        let input = "3 != 7";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(true), vm.stack[0]);
    }

    #[test]
    fn test_prefixes() {
        let input = "-2";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Int(-2), vm.stack[0]);

        let input = "!true";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(false), vm.stack[0]);

        let input = "!!true";
        let mut vm = Vm::new(compiled(input));
        vm.run();
        assert_eq!(Object::Boolean(true), vm.stack[0]);
    }
}
