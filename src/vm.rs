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
    fn test_enum() {
        let input = "7";
        let mut vm = Vm::new(compiled(input));
        vm.run();

        assert_eq!(Object::Int(7), vm.stack[0]);
    }
}
