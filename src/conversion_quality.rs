use crate::constants::*;
use std::cmp::Ordering;

/// The quality of a conversion. Each field represents whether that attribute is retained in a
/// conversion - for example if structure=true, a conversion retains a document's structure. For
/// conversions in which an attribute is non applicable, it should be set to true.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct ConversionQuality {
    /// In a data type like HTML, or JSON, whether the structural integrity stays the same
    pub structure: bool,
    /// Things like bold, italics, special formatting
    pub formatting: bool,
    /// If the data was human readable, will it still be?
    pub readability: bool,
    /// If the data was compressed, is it still?
    pub compression: bool,
    /// Is the ordering of the data retained?
    pub ordering: bool,
    /// Can we process this in a stream, without pulling the whole file into memory?
    pub streamability: bool
}

macro_rules! add_for {
    ($accum: ident, $($field: expr => $val: expr),*) => {
        $(
            if ($field) {
                $accum += $val
            }
        )*
    }
}

impl ConversionQuality {
    pub fn quality(&self) -> i32 {
        let mut qual: i32 = 0;
        add_for!(qual,
            self.structure => STRUCTURE_WEIGHTING,
            self.formatting => FORMATTING_WEIGHTING,
            self.readability => READABILITY_WEIGHTING,
            self.compression => COMPRESSION_WEIGHTING,
            self.ordering => ORDERING_WEIGHTING,
            self.streamability => STREAMABILITY_WEIGHTING
        );
        qual
    }
}

impl PartialOrd<Self> for ConversionQuality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quality().partial_cmp(&other.quality())
    }
}

impl Ord for ConversionQuality {
    fn cmp(&self, other: &Self) -> Ordering {
        self.quality().cmp(&other.quality())
    }
}