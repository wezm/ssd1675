// This is indeed a copy of Pimoroni's Inky LUTs. Saved here for reference.
// They license it out under the MIT license. Refer to LICENSE-MIT for more details.

// More documentation for LUTs can be found at https://github.com/pimoroni/inky/blob/master/library/inky/inky.py#L95
/// Standard refresh LUT for red, black and white displays.
#[rustfmt::skip]
pub const RBW: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b01001000, 0b10100000, 0b00010000, 0b00010000, 0b00010011, 0b00000000, 0b00000000,  // LUT0 - Black
    0b01001000, 0b10100000, 0b10000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000,  // LUT1 - White
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // IGNORE
    0b01001000, 0b10100101, 0b00000000, 0b10111011, 0b00000000, 0b00000000, 0b00000000,  // LUT3 - Red
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // LUT4 - VCOM

    // Duration            |  Repeat
    // A   B     C     D   |
    64,   12,   32,   12,    6,   // 0 Flash
    16,   8,    4,    4,     6,   // 1 clear
    4,    8,    8,    16,    16,  // 2 bring in the black
    2,    2,    2,    64,    32,  // 3 time for red
    2,    2,    2,    2,     2,   // 4 final black sharpen phase
    0,    0,    0,    0,     0,   // 5
    0,    0,    0,    0,     0    // 6
];

/// LUT for black and white displays. As far as I know, you can also use this on tricolour displays for faster refresh.
#[rustfmt::skip]
pub const BW: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b01001000, 0b10100000, 0b00010000, 0b00010000, 0b00010011, 0b00000000, 0b00000000,  // LUT0 - Black
    0b01001000, 0b10100000, 0b10000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000,  // LUT1 - White
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // IGNORE
    0b01001000, 0b10100101, 0b00000000, 0b10111011, 0b00000000, 0b00000000, 0b00000000,  // LUT3 - Red
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // LUT4 - VCOM

    // Duration            |  Repeat
    // A   B     C     D   |
    16,   4,    4,    4,     4,   // 0 Flash
    16,   4,    4,    4,     4,   // 1 clear
    4,    8,    8,   16,    16,   // 2 bring in the black
    0,    0,    0,    0,     0,   // 3 time for red
    0,    0,    0,    0,     0,   // 4 final black sharpen phase
    0,    0,    0,    0,     0,   // 5
    0,    0,    0,    0,     0    // 6
];

// There is also a yellow LUT on Pimoroni's repo, but I don't have a yellow display so ::shrugs::
