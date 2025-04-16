// Project tag
pub const PROJECT_TAG: &str = "Brollup";

// Port number
pub const SIGNET_PORT: u16 = 6272;
pub const MAINNET_PORT: u16 = 6272;

// Well-known coordinator key
pub const SIGNET_COORDINATOR: [u8; 32] = [
    0xa3, 0x08, 0xf8, 0x7d, 0x88, 0x7d, 0x78, 0x34, 0x19, 0xb8, 0x4b, 0x97, 0x65, 0x1f, 0xd8, 0xa5,
    0xf8, 0x8f, 0x6d, 0xb6, 0x41, 0x4a, 0xe6, 0xeb, 0x19, 0x84, 0xcc, 0x67, 0x42, 0xee, 0xf0, 0x9e,
];
pub const MAINNET_COORDINATOR: [u8; 32] = [
    0xa3, 0x08, 0xf8, 0x7d, 0x88, 0x7d, 0x78, 0x34, 0x19, 0xb8, 0x4b, 0x97, 0x65, 0x1f, 0xd8, 0xa5,
    0xf8, 0x8f, 0x6d, 0xb6, 0x41, 0x4a, 0xe6, 0xeb, 0x19, 0x84, 0xcc, 0x67, 0x42, 0xee, 0xf0, 0x9e,
];

// Initial operator set
pub const INITIAL_OPERATOR_SET: [[u8; 32]; 3] = [
    [
        0x33, 0xFA, 0x24, 0x46, 0x3F, 0xB5, 0xCB, 0xFB, 0x74, 0xA9, 0x19, 0x81, 0xC6, 0xC6, 0xCD,
        0x48, 0xEC, 0xD9, 0x0E, 0x8D, 0x5E, 0xA7, 0x35, 0x4F, 0x62, 0x2A, 0x17, 0xB5, 0x5B, 0x97,
        0x2C, 0x30,
    ],
    [
        0xD8, 0x9B, 0xDF, 0xBE, 0x0D, 0xB2, 0xC3, 0x07, 0xD9, 0x60, 0xF7, 0x64, 0x05, 0x49, 0xF6,
        0xBF, 0x01, 0x3D, 0x54, 0x9B, 0xE8, 0x35, 0xB8, 0xA4, 0x3B, 0x0B, 0xF4, 0x2C, 0xBC, 0xC9,
        0x65, 0x44,
    ],
    [
        0xA2, 0x14, 0x86, 0x0D, 0x7F, 0xBC, 0x32, 0xD1, 0xB0, 0x59, 0x3A, 0xC3, 0xD4, 0xA3, 0x08,
        0xA1, 0x69, 0x4A, 0x12, 0x24, 0xFB, 0x86, 0x0B, 0x8B, 0xC4, 0x39, 0x80, 0xF5, 0x31, 0xF9,
        0x75, 0x39,
    ],
];

// Sync start heights
pub const SIGNET_SYNC_START_HEIGHT: u64 = 244_066;
pub const MAINNET_SYNC_START_HEIGHT: u64 = 888_116;
