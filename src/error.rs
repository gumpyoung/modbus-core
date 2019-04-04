use core::fmt;

/// modbus-core Error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid coil value
    CoilValue(u16),
    /// Invalid buffer size
    BufferSize,
    /// Invalid function code
    FnCode(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            CoilValue(v) => write!(f, "Invalid coil value: {}", v),
            BufferSize => write!(f, "Invalid buffer size"),
            FnCode(fn_code) => write!(f, "Invalid function code: 0x{:0>2X}", fn_code),
        }
    }
}