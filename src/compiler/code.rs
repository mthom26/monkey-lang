pub enum OpCode {
    OpConstant(u16),
}

pub fn make_op(opcode: OpCode) -> Vec<u8> {
    match opcode {
        OpCode::OpConstant(operand) => {
            let mut output = vec![0x01];
            let int_one = (operand >> 8) as u8;
            let int_two = operand as u8;
            output.push(int_one);
            output.push(int_two);
            output
        }
    }
}

pub fn two_u8_to_usize(int_one: u8, int_two: u8) -> usize {
    ((int_one as usize) << 8) | int_two as usize
}

#[cfg(test)]
mod tests {
    use crate::compiler::{make_op, two_u8_to_usize, OpCode};

    #[test]
    fn make_op_constant() {
        let op = make_op(OpCode::OpConstant(65534));
        let expected = vec![0x01, 255, 254];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpConstant(4449));
        let expected = vec![0x01, 17, 97];
        assert_eq!(expected, op);
    }

    #[test]
    fn test_two_u8_to_usize() {
        let input = two_u8_to_usize(1, 1);
        let expected = 257;
        assert_eq!(expected, input);

        let input = two_u8_to_usize(10, 77);
        let expected = 2637;
        assert_eq!(expected, input);
    }
}
