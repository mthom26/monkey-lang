pub enum OpCode {
    OpConstant(u16),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPop,
    OpTrue,
    OpFalse,
    OpGreater,
    OpLess,
    OpEqual,
    OpNotEqual,
    OpBang,
    OpMinus,
    OpJmp(u16),
    OpJmpIfFalse(u16),
    OpSetGlobal(u16),
    OpGetGlobal(u16),
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
        OpCode::OpAdd => vec![0x02],
        OpCode::OpSub => vec![0x03],
        OpCode::OpMul => vec![0x04],
        OpCode::OpDiv => vec![0x05],
        OpCode::OpPop => vec![0x06],
        OpCode::OpTrue => vec![0x07],
        OpCode::OpFalse => vec![0x08],
        OpCode::OpGreater => vec![0x09],
        OpCode::OpLess => vec![0x0a],
        OpCode::OpEqual => vec![0x0b],
        OpCode::OpNotEqual => vec![0x0c],
        OpCode::OpBang => vec![0x0d],
        OpCode::OpMinus => vec![0x0e],
        OpCode::OpJmp(operand) => {
            let mut output = vec![0x0f];
            let int_one = (operand >> 8) as u8;
            let int_two = operand as u8;
            output.push(int_one);
            output.push(int_two);
            output
        }
        OpCode::OpJmpIfFalse(operand) => {
            let mut output = vec![0x10];
            let int_one = (operand >> 8) as u8;
            let int_two = operand as u8;
            output.push(int_one);
            output.push(int_two);
            output
        }
        OpCode::OpSetGlobal(operand) => {
            let mut output = vec![0x11];
            let int_one = (operand >> 8) as u8;
            let int_two = operand as u8;
            output.push(int_one);
            output.push(int_two);
            output
        }
        OpCode::OpGetGlobal(operand) => {
            let mut output = vec![0x12];
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
    fn make_ops() {
        let op = make_op(OpCode::OpAdd);
        let expected = vec![0x02];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpSub);
        let expected = vec![0x03];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpMul);
        let expected = vec![0x04];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpDiv);
        let expected = vec![0x05];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpPop);
        let expected = vec![0x06];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpTrue);
        let expected = vec![0x07];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpFalse);
        let expected = vec![0x08];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpGreater);
        let expected = vec![0x09];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpLess);
        let expected = vec![0x0a];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpEqual);
        let expected = vec![0x0b];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpNotEqual);
        let expected = vec![0x0c];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpBang);
        let expected = vec![0x0d];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpMinus);
        let expected = vec![0x0e];
        assert_eq!(expected, op);
    }

    #[test]
    fn test_jumps() {
        let op = make_op(OpCode::OpJmp(65534));
        let expected = vec![0x0f, 255, 254];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpJmpIfFalse(65534));
        let expected = vec![0x10, 255, 254];
        assert_eq!(expected, op);
    }

    #[test]
    fn test_globals() {
        let op = make_op(OpCode::OpSetGlobal(65534));
        let expected = vec![0x11, 255, 254];
        assert_eq!(expected, op);

        let op = make_op(OpCode::OpGetGlobal(65534));
        let expected = vec![0x12, 255, 254];
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
