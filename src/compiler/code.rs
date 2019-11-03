pub enum OpCode {
    OpConstant(u16),
}

pub fn make_op(opcode: OpCode) -> Vec<u8> {
    match opcode {
        OpCode::OpConstant(operand) => {
            let mut output = vec![0x01];
            let int1 = (operand >> 8) as u8;
            let int2 = operand as u8;
            output.push(int1);
            output.push(int2);
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::{make_op, OpCode};

    #[test]
    fn make_op_constant() {
        let op = make_op(OpCode::OpConstant(65534));
        let expected = vec![0x01, 255, 254];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpConstant(4449));
        let expected = vec![0x01, 17, 97];
        assert_eq!(expected, op);
    }
}
