//short aliases for fixed-width integer types
//and handy size constants.

pub type Byte = u8;
pub type Word = u16;
pub type DWord = u32;
pub type QWord = u64;

pub type SignedByte = i8;
pub type SignedWord = i16;
pub type SignedDWord = i32;
pub type SignedQWord = i64;

pub const KB: u32 = 1024;
pub const MB: u32 = KB * KB;
pub const GB: u32 = KB * KB * KB;