#[cfg(test)]
mod opcodec_tests {

    use brollup::executive::{
        opcode::{
            codec::{OpcodeDecoder, OpcodeEncoder},
            op::push::{
                op_10::OP_10, op_11::OP_11, op_12::OP_12, op_13::OP_13, op_14::OP_14, op_15::OP_15,
                op_16::OP_16, op_2::OP_2, op_3::OP_3, op_4::OP_4, op_5::OP_5, op_6::OP_6,
                op_7::OP_7, op_8::OP_8, op_9::OP_9, op_false::OP_FALSE, op_pushdata::OP_PUSHDATA,
                op_true::OP_TRUE,
            },
            opcode::Opcode,
        },
        stack::stack_error::StackError,
    };

    #[test]
    fn opcodec_test() -> Result<(), StackError> {
        let deadbeef = vec![0xde, 0xad, 0xbe, 0xef];
        let deadbeef_encoded = OP_PUSHDATA(deadbeef.clone()).encode();

        let mut opcodes_encoded: Vec<u8> = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10,
        ];

        opcodes_encoded.extend(deadbeef_encoded);

        let mut opcodes_byte_stream = opcodes_encoded.into_iter();

        // Decode 1st opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_FALSE(OP_FALSE));

        // Decode 2nd opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_TRUE(OP_TRUE));

        // Decode 3rd opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_2(OP_2));

        // Decode 4th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_3(OP_3));

        // Decode 5th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_4(OP_4));

        // Decode 6th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_5(OP_5));

        // Decode 7th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_6(OP_6));

        // Decode 8th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_7(OP_7));

        // Decode 9th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_8(OP_8));

        // Decode 10th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_9(OP_9));

        // Decode 11th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_10(OP_10));

        // Decode 12th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_11(OP_11));

        // Decode 13th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_12(OP_12));

        // Decode 14th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_13(OP_13));

        // Decode 15th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_14(OP_14));

        // Decode 16th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_15(OP_15));

        // Decode 17th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_16(OP_16));

        // Decode 18th opcode
        let decoded_opcode = OpcodeDecoder::decode(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_PUSHDATA(OP_PUSHDATA(deadbeef)));

        // Byte stream should be empty now
        assert_eq!(opcodes_byte_stream.len(), 0);

        Ok(())
    }
}
