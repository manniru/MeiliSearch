#![cfg_attr(feature = "nightly", feature(test))]

pub mod automaton;
pub mod database;
pub mod data;
pub mod rank;
pub mod tokenizer;
mod attribute;
mod word_area;
mod common_words;

pub use rocksdb;

pub use self::tokenizer::Tokenizer;
pub use self::common_words::CommonWords;
pub use self::attribute::{Attribute, AttributeError};
pub use self::word_area::{WordArea, WordAreaError};

/// Represent an internally generated document unique identifier.
///
/// It is used to inform the database the document you want to deserialize.
/// Helpful for custom ranking.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DocumentId(u64);

/// This structure represent the position of a word
/// in a document and its attributes.
///
/// This is stored in the map, generated at index time,
/// extracted and interpreted at search time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DocIndex {
    /// The document identifier where the word was found.
    pub document_id: DocumentId,

    /// The attribute in the document where the word was found
    /// along with the index in it.
    pub attribute: Attribute,

    /// The position in bytes where the word was found
    /// along with the length of it.
    ///
    /// It informs on the original word area in the text indexed
    /// without needing to run the tokenizer again.
    pub word_area: WordArea,
}

/// This structure represent a matching word with informations
/// on the location of the word in the document.
///
/// The order of the field is important because it defines
/// the way these structures are ordered between themselves.
///
/// The word in itself is not important.
// TODO do data oriented programming ? very arrays ?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Match {
    /// The word index in the query sentence.
    /// Same as the `attribute_index` but for the query words.
    ///
    /// Used to retrieve the automaton that match this word.
    pub query_index: u32,

    /// The distance the word has with the query word
    /// (i.e. the Levenshtein distance).
    pub distance: u8,

    /// The attribute in the document where the word was found
    /// along with the index in it.
    pub attribute: Attribute,

    /// Whether the word that match is an exact match or a prefix.
    pub is_exact: bool,

    /// The position in bytes where the word was found
    /// along with the length of it.
    ///
    /// It informs on the original word area in the text indexed
    /// without needing to run the tokenizer again.
    pub word_area: WordArea,
}

impl Match {
    pub fn zero() -> Self {
        Match {
            query_index: 0,
            distance: 0,
            attribute: Attribute::new_faillible(0, 0),
            is_exact: false,
            word_area: WordArea::new_faillible(0, 0),
        }
    }

    pub fn max() -> Self {
        Match {
            query_index: u32::max_value(),
            distance: u8::max_value(),
            attribute: Attribute::max_value(),
            is_exact: true,
            word_area: WordArea::max_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn docindex_mem_size() {
        assert_eq!(mem::size_of::<DocIndex>(), 16);
    }
}
