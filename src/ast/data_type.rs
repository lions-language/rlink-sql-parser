#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DataType {
    /// Fixed-length character type e.g. CHAR(10) / CHAR()
    Char(Option<u64>),
    /// Variable-length character type e.g. VARCHAR(10) / VARCHAR()
    Varchar(Option<u64>),
    /// Uuid type
    Uuid,
    /// Large character object e.g. CLOB(1000)
    Clob(u64),
}
