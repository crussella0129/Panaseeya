//! Title normalisation: deciding when two titles name the same game.
//!
//! Matching is exact on the *sequence of words* in a title. A word is a
//! maximal run of letters and digits, compared without regard to case. Every
//! other character — punctuation, trademark symbols, whitespace — only
//! separates words. So `Half-Life` and `half life` match, while `Meier's` and
//! `Meiers` do not, because the apostrophe splits one word into two.

/// Returns the key under which a title is grouped.
///
/// Two titles belong to the same game exactly when their keys are equal.
///
/// A title with no letters or digits (for example `!!!`) has no words, so its
/// key falls back to the trimmed, lower-cased original. A blank title yields an
/// empty key; the catalog rejects blank titles before they reach storage.
pub fn normalize_title(title: &str) -> String {
    let lowered = title.to_lowercase();
    let mut key = String::with_capacity(lowered.len());
    for word in lowered
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
    {
        if !key.is_empty() {
            key.push(' ');
        }
        key.push_str(word);
    }
    if key.is_empty() {
        lowered.trim().to_owned()
    } else {
        key
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_title;

    #[test]
    fn normalize_merges_case_punctuation_trademark_whitespace() {
        let same = [
            ("DOOM", "Doom"),
            ("Half-Life", "Half Life"),
            (
                "Sid Meier's Civilization® VI",
                "Sid Meier’s Civilization VI",
            ),
            ("  Hades  ", "Hades"),
            ("The Witcher 3: Wild Hunt", "The Witcher 3 - Wild Hunt"),
            ("Control™", "Control"),
        ];
        for (left, right) in same {
            assert_eq!(
                normalize_title(left),
                normalize_title(right),
                "{left:?} and {right:?} should group"
            );
        }
    }

    #[test]
    fn normalize_keeps_different_words_apart() {
        let different = [
            ("Portal", "Portal 2"),
            ("HalfLife", "Half Life"),
            ("Hades", "Hades II"),
            ("Doom", "Doom Eternal"),
            ("Sid Meier's Pirates", "Sid Meiers Pirates"),
        ];
        for (left, right) in different {
            assert_ne!(
                normalize_title(left),
                normalize_title(right),
                "{left:?} and {right:?} should stay apart"
            );
        }
    }

    #[test]
    fn normalize_falls_back_when_only_punctuation() {
        assert_eq!(normalize_title("  !!!  "), "!!!");
    }
}
