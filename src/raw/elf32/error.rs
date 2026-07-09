use std::fmt;
#[derive(Debug)]
pub enum Error {
    FileReadError,
    HeaderParsingError,
    FieldBuildingError,
    InvalidFieldValue,
    InvalidMagic,
    InvalidElfClass,
    InvalidElfEndian,
    ByteParsingError,
    InvalidShEntSize,
    InvalidPhEntSize,
    IndexOutOfBoundsError,
    SectionHeaderConstructionError,
    ProgramHeaderConstructionError,
    CalcOffsetError,
    SectionHeaderRetrievalError,
    ProgramHeaderRetrievalError,
    SectionRetrievalError,
    InvalidSegmentMemSz,
    NotTable,
    SymbolConstructionError,
    NotSymbolTable,
    NotRelTable,
    NotRelaTable,
    RelConstructionError,
    RelaConstructionError,
    NotStringTable,
    SectionbuildingError,
    SectionNameFetchingError,
    SymbolNameFetchingError,
    SymbolFetchingError,
    RelFetchingError,
    RelaFetchingError,
    SymFetchingError,
    BufferTooShort,
    OnceCellFailureError,
    NoAssociatedSectionError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldBuildingError => write!(f, "FileReadError"),
            Self::HeaderParsingError => {
                write!(f, "HeaderParsingError")
            }
            Self::FileReadError => write!(f, "FileReadError"),
            Self::InvalidFieldValue => write!(f, "InvalidFieldValue"),
            Self::InvalidMagic => write!(f, "InvalidMagic"),
            Self::InvalidElfClass => write!(f, "InvalidElfClass"),
            Self::InvalidElfEndian => write!(f, "InvalidElfEndian"),
            Self::ByteParsingError => write!(f, "ByteParsingError"),
            Self::InvalidShEntSize => write!(f, "InvalidShEntSize"),
            Self::InvalidPhEntSize => write!(f, "InvalidPhEntSize"),
            Self::IndexOutOfBoundsError => {
                write!(f, "IndexOutOfBoundsError")
            }

            Self::SectionHeaderConstructionError => {
                write!(f, "SectionHeaderConstructionError")
            }

            Self::CalcOffsetError => write!(f, "CalcOffsetError"),

            Self::SectionHeaderRetrievalError => {
                write!(f, "SectionHeaderRetrievalError")
            }
            Self::ProgramHeaderRetrievalError => {
                write!(f, "ProgramHeaderRetrievalError")
            }
            Self::SectionRetrievalError => {
                write!(f, "SectionRetrievalError")
            }

            Self::InvalidSegmentMemSz => {
                write!(f, "InvalidSegmentMemSz")
            }

            Self::ProgramHeaderConstructionError => {
                write!(f, "ProgramHeaderConstructionError")
            }
            Self::NotTable => write!(f, "NotTable"),

            Self::SymbolConstructionError => {
                write!(f, "SymbolConstructionError")
            }

            Self::NotSymbolTable => write!(f, "NotSymbolTable"),
            Self::NotRelTable => write!(f, "NotRelTable"),
            Self::NotRelaTable => write!(f, "NotRelaTable"),
            Self::RelConstructionError => {
                write!(f, "RelConstructionError")
            }
            Self::RelaConstructionError => {
                write!(f, "RelaConstructionError")
            }
            Self::NotStringTable => write!(f, "NotStringTable"),
            Self::SectionbuildingError => {
                write!(f, "SectionbuildingError")
            }
            Self::SectionNameFetchingError => {
                write!(f, "SectionNameFetchingError")
            }
            Self::SymbolNameFetchingError => {
                write!(f, "SymbolNameFetchingError")
            }
            Self::SymbolFetchingError => {
                write!(f, "SymbolFetchingError")
            }
            Self::SymFetchingError => write!(f, "SymFetchingError"),
            Self::RelFetchingError => write!(f, "RelFetchingError"),
            Self::RelaFetchingError => write!(f, "RelaFetchingError"),
            Self::BufferTooShort => write!(f, "BufferTooShort"),
            Self::OnceCellFailureError => {
                write!(f, "OnceCellFailureError")
            }
            Self::NoAssociatedSectionError => {
                write!(f, "NoAssociatedSectionError")
            }
        }
    }
}
