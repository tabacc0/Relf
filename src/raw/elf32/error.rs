use std::fmt;
pub enum Error {
    FileReadError,
    HeaderParsingError,
    FieldBuildingError,
    InvalidFieldValue,
    InvalidMagic,
    InvalidElfClass,
    InvalidElfEndian,
}

impl fmt::Display for Error {
    fn fmt(&self,f :&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldBuildingError => write!(f,"FileReadError"),
            Self::HeaderParsingError => write!(f,"HeaderParsingError"),
            Self::FileReadError => write!(f,"FileReadError"),
            Self::InvalidFieldValue => write!(f,"InvalidFieldValue"),
            Self::InvalidMagic => write!(f,"InvalidMagic"),
            Self::InvalidElfClass => write!(f,"InvalidElfClass"),
            Self::InvalidElfEndian => write!(f,"InvalidElfEndian"),
        }
    }
}
