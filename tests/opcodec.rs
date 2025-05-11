#[cfg(test)]
mod opcodec_tests {
    use cube::executive::{
        opcode::{
            compiler::{compiler::OpcodeCompiler, compiler_error::OpcodeDecompileError},
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

    /// Minimal OP_PUSHDATA encoding tests.
    #[test]
    fn test_minimal_encoding() -> Result<(), StackError> {
        // Test Empty Pushdata
        let pushdata = OP_PUSHDATA(vec![]);
        assert!(pushdata.compiled_bytes().unwrap() == vec![0x00]); // Encoded as OP_FALSE
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_FALSE::bytecode());

        // Test 0
        let pushdata = OP_PUSHDATA(vec![0x00]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x00]); // Encoded as OP_FALSE
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_FALSE::bytecode());

        // Test 1
        let pushdata = OP_PUSHDATA(vec![0x01]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x51]); // Encoded as OP_TRUE
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_TRUE::bytecode());

        // Test 2
        let pushdata = OP_PUSHDATA(vec![0x02]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x52]); // Encoded as OP_2
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_2::bytecode());

        // Test 3
        let pushdata = OP_PUSHDATA(vec![0x03]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x53]); // Encoded as OP_3
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_3::bytecode());

        // Test 4
        let pushdata = OP_PUSHDATA(vec![0x04]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x54]); // Encoded as OP_4
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_4::bytecode());

        // Test 5
        let pushdata = OP_PUSHDATA(vec![0x05]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x55]); // Encoded as OP_5
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_5::bytecode());

        // Test 6
        let pushdata = OP_PUSHDATA(vec![0x06]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x56]); // Encoded as OP_6
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_6::bytecode());

        // Test 7
        let pushdata = OP_PUSHDATA(vec![0x07]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x57]); // Encoded as OP_7
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_7::bytecode());

        // Test 8
        let pushdata = OP_PUSHDATA(vec![0x08]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x58]); // Encoded as OP_8
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_8::bytecode());

        // Test 9
        let pushdata = OP_PUSHDATA(vec![0x09]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x59]); // Encoded as OP_9
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_9::bytecode());

        // Test 10
        let pushdata = OP_PUSHDATA(vec![0x0a]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x5a]); // Encoded as OP_10
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_10::bytecode());

        // Test 11
        let pushdata = OP_PUSHDATA(vec![0x0b]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x5b]); // Encoded as OP_11
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_11::bytecode());

        // Test 12
        let pushdata = OP_PUSHDATA(vec![0x0c]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x5c]); // Encoded as OP_12
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_12::bytecode());

        // Test 13
        let pushdata = OP_PUSHDATA(vec![0x0d]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x5d]); // Encoded as OP_13
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_13::bytecode());

        // Test 14
        let pushdata = OP_PUSHDATA(vec![0x0e]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x5e]); // Encoded as OP_14
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_14::bytecode());

        // Test 15
        let pushdata = OP_PUSHDATA(vec![0x0f]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x5f]); // Encoded as OP_15
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_15::bytecode());

        // Test 16
        let pushdata = OP_PUSHDATA(vec![0x10]);

        assert!(pushdata.compiled_bytes().unwrap() == vec![0x60]); // Encoded as OP_16
        assert_eq!(pushdata.compiled_bytes().unwrap(), OP_16::bytecode());

        // Test 17
        let pushdata = OP_PUSHDATA(vec![0x11]);

        assert_eq!(pushdata.compiled_bytes().unwrap(), vec![0x01, 0x11]); // Encoded as OP_PUSHDATA tier 1.

        Ok(())
    }

    /// Non-minimal OP_PUSHDATA decoding tests.
    #[test]
    fn test_non_minimal_decoding() -> Result<(), StackError> {
        {
            // Non minimal 0x00 push decoding must fail.
            let data = vec![0x01, 0x00];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_FALSE (OP_0).
            let data = vec![0x00];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x01 push decoding must fail.
            let data = vec![0x01, 0x01];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_TRUE (OP_1).
            let data = vec![0x51];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x02 push decoding must fail.
            let data = vec![0x01, 0x02];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_2 (OP_2).
            let data = vec![0x52];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x03 push decoding must fail.
            let data = vec![0x01, 0x03];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_3 (OP_3).
            let data = vec![0x53];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x04 push decoding must fail.
            let data = vec![0x01, 0x04];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_4 (OP_4).
            let data = vec![0x54];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x05 push decoding must fail.
            let data = vec![0x01, 0x05];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_5 (OP_5).
            let data = vec![0x55];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x06 push decoding must fail.
            let data = vec![0x01, 0x06];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_6 (OP_6).
            let data = vec![0x56];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x07 push decoding must fail.
            let data = vec![0x01, 0x07];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_7 (OP_7).
            let data = vec![0x57];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x08 push decoding must fail.
            let data = vec![0x01, 0x08];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_8 (OP_8).
            let data = vec![0x58];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x09 push decoding must fail.
            let data = vec![0x01, 0x09];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_9 (OP_9).
            let data = vec![0x59];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x0a push decoding must fail.
            let data = vec![0x01, 0x0a];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_10 (OP_10).
            let data = vec![0x5a];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x0b push decoding must fail.
            let data = vec![0x01, 0x0b];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_11 (OP_11).
            let data = vec![0x5b];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x0c push decoding must fail.
            let data = vec![0x01, 0x0c];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_12 (OP_12).
            let data = vec![0x5c];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x0d push decoding must fail.
            let data = vec![0x01, 0x0d];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_13 (OP_13).
            let data = vec![0x5d];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x0e push decoding must fail.
            let data = vec![0x01, 0x0e];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_14 (OP_14).
            let data = vec![0x5e];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x0f push decoding must fail.
            let data = vec![0x01, 0x0f];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_15 (OP_15).
            let data = vec![0x5f];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // Non minimal 0x10 push decoding must fail.
            let data = vec![0x01, 0x10];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            match decoded_opcode {
                Ok(_) => panic!("Expected error"),
                Err(e) => assert_eq!(e, OpcodeDecompileError::NonMinimalDataPushError),
            }

            // Should have been encoded as OP_16 (OP_16).
            let data = vec![0x60];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        {
            // 0x11 push decoding must pass.
            let data = vec![0x01, 0x11];
            let mut byte_stream = data.into_iter();
            let decoded_opcode = Opcode::decompile(&mut byte_stream);
            assert!(decoded_opcode.is_ok());
        }

        Ok(())
    }

    /// Non-minimal, multi-tier OP_PUSHDATA encoding tests.
    #[test]
    fn test_pushdata_multi_tier_encoding() -> Result<(), StackError> {
        // Test 1 byte.
        let pushdata = OP_PUSHDATA(vec![0xff]);
        let mut expected_encoded = Vec::<u8>::new();
        expected_encoded.push(0x01);
        expected_encoded.extend(vec![0xff]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 0.

        // Test 2 bytes.
        let pushdata = OP_PUSHDATA(vec![0xde, 0xad]);
        let mut expected_encoded = Vec::<u8>::new();
        expected_encoded.push(0x02); // Data length.
        expected_encoded.extend(vec![0xde, 0xad]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 0.

        // Test 3 bytes.
        let pushdata = OP_PUSHDATA(vec![0xde, 0xad, 0xbe]);
        let mut expected_encoded = Vec::<u8>::new();
        expected_encoded.push(0x03); // Data length.
        expected_encoded.extend(vec![0xde, 0xad, 0xbe]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 0.

        // Test 10 bytes.
        let pushdata = OP_PUSHDATA(vec![
            0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
        ]);
        let mut expected_encoded = Vec::<u8>::new();
        expected_encoded.push(0x0a); // Data length.
        expected_encoded.extend(vec![
            0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
        ]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 0.

        // Test 74 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 74]);
        let mut expected_encoded = Vec::<u8>::new();
        expected_encoded.push(0x4a); // Data length.
        expected_encoded.extend(vec![0xff; 74]);

        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 0.

        // Test 75 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 75]);
        let mut expected_encoded = Vec::<u8>::new();
        expected_encoded.push(0x4b); // Data length.
        expected_encoded.extend(vec![0xff; 75]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 0.

        // Test 76 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 76]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 1 byte.
        expected_encoded.push(0x4c);
        // Data length.
        expected_encoded.push(76);
        expected_encoded.extend(vec![0xff; 76]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 1.

        // Test 77 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 77]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 1 byte.
        expected_encoded.push(0x4c);
        // Data length.
        expected_encoded.push(77);
        expected_encoded.extend(vec![0xff; 77]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 1.

        // Test 254 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 254]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 1 byte.
        expected_encoded.push(0x4c);
        // Data length.
        expected_encoded.push(254);
        expected_encoded.extend(vec![0xff; 254]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 1.

        // Test 255 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 255]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 1 byte.
        expected_encoded.push(0x4c);
        // Data length.
        expected_encoded.push(255);
        expected_encoded.extend(vec![0xff; 255]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 1.

        // Test 256 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 256]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 2 byte.
        expected_encoded.push(0x4d);
        // 256 is 0x0100 in little endian.
        let length_bytes: [u8; 2] = {
            let mut length_bytes = [0u8; 2];
            length_bytes[0] = (256 & 0xff) as u8;
            length_bytes[1] = (256 >> 8) as u8;
            length_bytes
        };
        expected_encoded.extend(length_bytes);
        expected_encoded.extend(vec![0xff; 256]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 2.

        // Test 257 bytes.
        let pushdata = OP_PUSHDATA(vec![0xff; 257]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 2 byte.
        expected_encoded.push(0x4d);
        // 257 is 0x0101 in little endian.
        let length_bytes: [u8; 2] = {
            let mut length_bytes = [0u8; 2];
            length_bytes[0] = (257 & 0xff) as u8;
            length_bytes[1] = (257 >> 8) as u8;
            length_bytes
        };
        expected_encoded.extend(length_bytes);
        expected_encoded.extend(vec![0xff; 257]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 2.

        // Test 4095 bytes (MAX STACK ITEM SIZE).
        let pushdata = OP_PUSHDATA(vec![0xff; 4095]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 2 byte.
        expected_encoded.push(0x4d);
        // 4095 is 0x0fff in little endian.
        let length_bytes: [u8; 2] = {
            let mut length_bytes = [0u8; 2];
            length_bytes[0] = (4095 & 0xff) as u8;
            length_bytes[1] = (4095 >> 8) as u8;
            length_bytes
        };
        expected_encoded.extend(length_bytes);
        expected_encoded.extend(vec![0xff; 4095]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 2.

        // Test 65534 bytes (MAX TIER 2 SIZE -1).
        // This will be valid to encode/decode, but the opcode execution will fail (due to stack item size limit).
        let pushdata = OP_PUSHDATA(vec![0xff; 65534]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 2 byte.
        expected_encoded.push(0x4d);
        // 65534 is 0xfffe in little endian.
        let length_bytes: [u8; 2] = {
            let mut length_bytes = [0u8; 2];
            length_bytes[0] = (65534 & 0xff) as u8;
            length_bytes[1] = (65534 >> 8) as u8;
            length_bytes
        };
        expected_encoded.extend(length_bytes);
        expected_encoded.extend(vec![0xff; 65534]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 2.

        // Test 65535 bytes (MAX TIER 2 SIZE).
        // This will be valid to encode/decode, but the opcode execution will fail (due to stack item size limit).
        let pushdata = OP_PUSHDATA(vec![0xff; 65535]);
        let mut expected_encoded = Vec::<u8>::new();
        // Tier 2 byte.
        expected_encoded.push(0x4d);
        // 65535 is 0xffff in little endian.
        let length_bytes: [u8; 2] = {
            let mut length_bytes = [0u8; 2];
            length_bytes[0] = (65535 & 0xff) as u8;
            length_bytes[1] = (65535 >> 8) as u8;
            length_bytes
        };
        expected_encoded.extend(length_bytes);
        expected_encoded.extend(vec![0xff; 65535]);
        assert_eq!(pushdata.compiled_bytes().unwrap(), expected_encoded); // Encoded as OP_PUSHDATA tier 2.

        // Testing 65536 bytes should fail. Tier 3 (>65535 bytes) is not supported.
        let pushdata = OP_PUSHDATA(vec![0xff; 65536]);
        assert!(pushdata.compiled_bytes().is_none());

        Ok(())
    }

    #[test]
    fn opcode_encode_decode_test() -> Result<(), StackError> {
        // Encode the opcodes.
        let mut opcodes_encoded = Vec::<u8>::new();

        opcodes_encoded.extend(OP_FALSE::bytecode());
        opcodes_encoded.extend(OP_TRUE::bytecode());
        opcodes_encoded.extend(OP_2::bytecode());
        opcodes_encoded.extend(OP_3::bytecode());
        opcodes_encoded.extend(OP_4::bytecode());
        opcodes_encoded.extend(OP_5::bytecode());
        opcodes_encoded.extend(OP_6::bytecode());
        opcodes_encoded.extend(OP_7::bytecode());
        opcodes_encoded.extend(OP_8::bytecode());
        opcodes_encoded.extend(OP_9::bytecode());
        opcodes_encoded.extend(OP_10::bytecode());
        opcodes_encoded.extend(OP_11::bytecode());
        opcodes_encoded.extend(OP_12::bytecode());
        opcodes_encoded.extend(OP_13::bytecode());
        opcodes_encoded.extend(OP_14::bytecode());
        opcodes_encoded.extend(OP_15::bytecode());
        opcodes_encoded.extend(OP_16::bytecode());
        opcodes_encoded.extend(
            OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef])
                .compiled_bytes()
                .unwrap(),
        );

        let mut opcodes_byte_stream = opcodes_encoded.into_iter();

        // Decode 1st opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_FALSE(OP_FALSE));

        // Decode 2nd opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_TRUE(OP_TRUE));

        // Decode 3rd opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_2(OP_2));

        // Decode 4th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_3(OP_3));

        // Decode 5th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_4(OP_4));

        // Decode 6th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_5(OP_5));

        // Decode 7th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_6(OP_6));

        // Decode 8th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_7(OP_7));

        // Decode 9th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_8(OP_8));

        // Decode 10th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_9(OP_9));

        // Decode 11th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_10(OP_10));

        // Decode 12th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_11(OP_11));

        // Decode 13th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_12(OP_12));

        // Decode 14th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_13(OP_13));

        // Decode 15th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_14(OP_14));

        // Decode 16th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_15(OP_15));

        // Decode 17th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(decoded_opcode, Opcode::OP_16(OP_16));

        // Decode 18th opcode
        let decoded_opcode = Opcode::decompile(&mut opcodes_byte_stream).unwrap();
        assert_eq!(
            decoded_opcode,
            Opcode::OP_PUSHDATA(OP_PUSHDATA(vec![0xde, 0xad, 0xbe, 0xef]))
        );

        // Byte stream should be empty now
        assert_eq!(opcodes_byte_stream.len(), 0);

        Ok(())
    }
}
