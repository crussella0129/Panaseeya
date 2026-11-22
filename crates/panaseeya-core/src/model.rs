//! The catalog's vocabulary: what an ownership record is made of.

use crate::error::Error;

/// The form in which a copy of a game is owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Medium {
    Digital,
    Physical,
    Rom,
}

impl Medium {
    /// The text form used in the database and in listings.
    pub fn as_str(self) -> &'static str {
        match self {
            Medium::Digital => "digital",
            Medium::Physical => "physical",
            Medium::Rom => "rom",
        }
    }
}

impl std::fmt::Display for Medium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Medium {
    type Err = Error;

    /// Parses exactly `digital`, `physical`, or `rom`.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "digital" => Ok(Medium::Digital),
            "physical" => Ok(Medium::Physical),
            "rom" => Ok(Medium::Rom),
            other => Err(Error::InvalidMedium {
                value: other.to_owned(),
            }),
        }
    }
}

/// How an ownership record entered the catalog.
///
/// Keeping this apart from the source name means a hand-entered record whose
/// label happens to be `epic` never collides with records imported from Epic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Origin {
    /// Imported from a store's own tooling.
    Store,
    /// Entered by hand.
    Manual,
}

impl Origin {
    /// The text form used in the database and in listings.
    pub fn as_str(self) -> &'static str {
        match self {
            Origin::Store => "store",
            Origin::Manual => "manual",
        }
    }
}

/// One copy of a game, ready to be recorded in the catalog.
///
/// A record is identified by its `origin`, `source`, and `source_id` together;
/// recording the same identity twice changes nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewOwnership {
    pub origin: Origin,
    /// Where the copy comes from: a store name such as `epic`, or a label the
    /// person chose for a manual entry.
    pub source: String,
    /// The source's own identifier for the game.
    pub source_id: String,
    pub medium: Medium,
    /// The game's title as the source spells it.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medium_parse_rejects_unknown_value() {
        let err = "cartridge".parse::<Medium>().unwrap_err();

        assert!(matches!(&err, Error::InvalidMedium { value } if value == "cartridge"));
        let message = err.to_string();
        for accepted in ["physical", "digital", "rom"] {
            assert!(message.contains(accepted), "{message}");
            assert_eq!(accepted.parse::<Medium>().unwrap().as_str(), accepted);
        }
    }
}
