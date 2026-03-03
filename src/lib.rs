//! A library for parsing IPA phonemes into their features and modifiers.
//! This is a work in progress, and the API is not yet stable. The main function is `Phoneme::from`,
//! which takes an IPA string and returns a `Phoneme` struct containing the features and modifiers of
//! the phoneme, as well as any warnings that were generated during parsing.

use std::collections::{HashSet, VecDeque};
use unicode_normalization::UnicodeNormalization;

pub mod data;
pub mod feature;
use data::{PHONEMES, POLYPHTHONG_COMPONENTS, POSTFIX_MODIFIERS, PREFIX_MODIFIERS, TONE_LETTERS};
use feature::{ConsonantFeature, Depth, Feature, Height, Manner, Modifier, Place, VowelFeature};

use crate::{
    data::{CLICKS, IMPLIED_MODIFIERS, PRENASALIZED_STOP_CONFUSABLE, PRENASALIZED_STOPS},
    feature::{PhonemeClass, Tone},
};

#[derive(Clone, Debug)]
/// A struct representing a phoneme.
///
/// Not all features and modifiers are guaranteed to be present, and some may be mutually exclusive.
/// The parsed version of a phoneme is not guaranteed to be stable across different versions of this library.
pub struct Phoneme<T: Into<String> + Clone> {
    /// Phonological features.
    features: HashSet<Feature>,
    /// Modifiers, usually diacritics. Does not include tone letters, which are stored separately.
    modifiers: HashSet<Modifier>,
    /// If this is a polyphthong, on-glides will be stored here.
    /// Polyphthongs must be represented with on-glides having the non-syllabic marker.
    on_glides: Vec<&'static [Feature]>,
    /// If this is a polyphthong, off-glides will be stored here.
    /// Polyphthongs must be represented with off-glides having the non-syllabic marker.
    off_glides: Vec<&'static [Feature]>,
    /// Tone letters will be stored here, in the order they appear in the IPA string.
    /// This does not include all possible tone modifiers - just the set ˩ ˨ ˧ ˦ ˥.
    tone_letters: Vec<Tone>,
    /// Whether this is a consonant or vowel.
    /// (If we couldn't find any features at all, this won't be known.)
    phoneme_class: Option<PhonemeClass>,
    // todo: make borrowed?
    /// The original IPA representation of the phoneme.
    representation: T,
}

impl<T: Into<String> + Clone> Phoneme<T> {
    /// Parses an IPA string into a `Phoneme` struct in a best-effort manner.
    /// Returns any warnings generated during parsing.
    pub fn from(ipa: T) -> (Self, Vec<&'static str>) {
        let mut features = HashSet::new();
        let mut modifiers = HashSet::new();
        let mut on_glides = Vec::new();
        let mut off_glides = Vec::new();
        let mut warnings = Vec::new();
        let representation = ipa.clone();

        // decompose the string as much as possible
        let mut ipa: &str = &ipa.into().nfd().collect::<String>();

        // let's pray that the compiler is smart enough to make this a `goto`
        let mut found_phoneme = false;

        // strip prefix modifiers
        'prefix_loop: loop {
            for (prefix, modifier) in PREFIX_MODIFIERS {
                if let Some(rest) = ipa.strip_prefix(prefix) {
                    if (*prefix == "ᵍ" || *prefix == "ᵏ")
                        && CLICKS.iter().any(|c| rest.starts_with(c))
                    {
                        break 'prefix_loop;
                    }
                    if PRENASALIZED_STOP_CONFUSABLE.contains(prefix) {
                        for (prenasalized_stop, f) in PRENASALIZED_STOPS {
                            if let Some(r) = ipa.strip_prefix(prenasalized_stop) {
                                features.extend(*f);
                                modifiers.insert(Modifier::PreNasalized);
                                ipa = r;
                                found_phoneme = true;
                                break 'prefix_loop;
                            }
                        }
                    }
                    modifiers.insert(*modifier);
                    ipa = rest;
                    continue 'prefix_loop;
                }
            }
            break;
        }

        if !found_phoneme {
            // strip on-glides
            'onglide_loop: loop {
                for (onglide, features) in POLYPHTHONG_COMPONENTS {
                    if let Some(rest) = ipa.strip_prefix(onglide) {
                        on_glides.push(*features);
                        ipa = rest;
                        continue 'onglide_loop;
                    }
                }
                break;
            }

            // find base phoneme
            for (phoneme, f) in PHONEMES {
                if let Some(rest) = ipa.strip_prefix(phoneme) {
                    features.extend(f.iter().copied());
                    ipa = rest;
                    if let Some(m) = IMPLIED_MODIFIERS.get(phoneme) {
                        modifiers.extend(m.iter());
                    }
                    break;
                }
            }
        }

        'offglide_loop_1: loop {
            for (offglide, features) in POLYPHTHONG_COMPONENTS {
                if let Some(rest) = ipa.strip_suffix(offglide) {
                    off_glides.push(*features);
                    ipa = rest;
                    continue 'offglide_loop_1;
                }
            }
            break;
        }

        'postfix_loop: loop {
            for (postfix, modifier) in POSTFIX_MODIFIERS {
                if let Some(rest) = ipa.strip_suffix(postfix) {
                    modifiers.insert(*modifier);
                    ipa = rest;
                    continue 'postfix_loop;
                }
            }
            break;
        }

        'offglide_loop_2: loop {
            for (offglide, features) in POLYPHTHONG_COMPONENTS {
                if let Some(rest) = ipa.strip_suffix(offglide) {
                    off_glides.push(*features);
                    ipa = rest;
                    continue 'offglide_loop_2;
                }
            }
            break;
        }

        let mut tone_letters = Vec::new();
        'tone_letter_loop: loop {
            for (tone_letter, modifier) in TONE_LETTERS {
                if let Some(rest) = ipa.strip_suffix(tone_letter) {
                    tone_letters.push(*modifier);
                    ipa = rest;
                    continue 'tone_letter_loop;
                }
            }
            break;
        }
        // since we strip the tone letters as suffixes, they will be in reverse order, so reverse them back
        tone_letters.reverse();

        if !ipa.is_empty() {
            warnings.push("leftover characters after parsing phoneme");
        }

        let phoneme_class = features.iter().next().map(Feature::phoneme_class);

        let phoneme = Self {
            features,
            modifiers,
            on_glides,
            off_glides,
            representation,
            tone_letters,
            phoneme_class,
        };

        (phoneme, warnings)
    }

    /// Get the phonological features of this phoneme.
    pub fn features(&self) -> &HashSet<Feature> {
        &self.features
    }

    /// Get the modifiers of this phoneme.
    pub fn modifiers(&self) -> &HashSet<Modifier> {
        &self.modifiers
    }

    /// Get the original IPA representation of this phoneme.
    ///
    /// This is _always_ the same as the input to `Phoneme::from`.
    pub fn representation(&self) -> &T {
        &self.representation
    }

    /// Get the on-glides of this phoneme, if it is a polyphthong.
    pub fn on_glides(&self) -> &Vec<&'static [Feature]> {
        &self.on_glides
    }

    /// Get the off-glides of this phoneme, if it is a polyphthong.
    pub fn off_glides(&self) -> &Vec<&'static [Feature]> {
        &self.off_glides
    }

    /// Get the tone letters of this phoneme, if any.
    ///
    /// This is in the order they appear in the IPA string.
    /// This does not include tone modifiers which appear on the vowel itself.
    /// Additionally, notation varies on what tone letters represent; for example, ExtraHigh may be used for a high level tone,
    /// or the notation may be based on chinese tone marking, in which e.g. "3" means a falling-rising tone. This will instead be parsed
    /// as Mid.
    // (which is sad, because that's the best tone...)
    pub fn tone_letters(&self) -> &Vec<Tone> {
        &self.tone_letters
    }

    pub fn class(&self) -> Option<PhonemeClass> {
        self.phoneme_class
    }

    pub fn is_consonant(&self) -> bool {
        self.phoneme_class == Some(PhonemeClass::Consonant)
    }

    pub fn is_vowel(&self) -> bool {
        self.phoneme_class == Some(PhonemeClass::Vowel)
    }

    pub fn name(&self) -> String {
        let expected_parts = self.features.len() + self.modifiers.len() + 1;
        let mut parts = VecDeque::with_capacity(expected_parts);

        let has_voicing_modifier = self.modifiers.iter().any(|m| matches!(m, Modifier::Voice(_)));

        let mut features: Vec<_> = if has_voicing_modifier {
            self.features
                .iter()
                .copied()
                .filter(|f| !matches!(f, Feature::Consonant(ConsonantFeature::Voiced)))
                .collect()
        } else {
            self.features.iter().copied().collect()
        };
        features.sort();
        for feature in features {
            parts.push_back(feature.into());
        }

        if !has_voicing_modifier && self.is_consonant() && !self.features.iter().any(|f| matches!(f, Feature::Consonant(ConsonantFeature::Voiced))) {
            parts.push_front("voiceless");
        }

        if self.phoneme_class == Some(PhonemeClass::Vowel) {
            parts.push_back("vowel")
        }

        for modifier in &self.modifiers {
            modifier.apply_modifier(&mut parts);
        }
        let mut name = parts.make_contiguous().join(" ");

        if !self.tone_letters.is_empty() {
            name.push_str(" with tone pattern ");
            for tone_letter in &self.tone_letters {
                name.push_str(tone_letter.as_number_str());
            }
        }

        name
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::Tone;

    use super::*;

    #[test]
    fn parsing_phoneme() {
        let (phoneme, warnings) = Phoneme::from("tʰ");
        assert!(warnings.is_empty());
        assert!(
            phoneme
                .features()
                .contains(&Feature::Consonant(ConsonantFeature::Manner(Manner::Stop)))
        );
        assert!(
            phoneme
                .features()
                .contains(&Feature::Consonant(ConsonantFeature::Place(
                    Place::Alveolar
                )))
        );
        assert!(phoneme.modifiers().contains(&Modifier::Aspirated));
        assert_eq!(*(phoneme.representation()), "tʰ");
    }

    #[test]
    fn parsing_polyphthong() {
        let (phoneme, warnings) = Phoneme::from("aɪ̯");
        assert!(warnings.is_empty());
        assert!(
            phoneme
                .features()
                .contains(&Feature::Vowel(VowelFeature::Height(Height::Open)))
        );
        assert!(
            phoneme
                .features()
                .contains(&Feature::Vowel(VowelFeature::Depth(Depth::Front)))
        );
        assert!(phoneme.on_glides().is_empty());
        assert_eq!(
            phoneme.off_glides(),
            &vec![&[
                Feature::Vowel(VowelFeature::Height(Height::NearClose)),
                Feature::Vowel(VowelFeature::Depth(Depth::NearFront))
            ]]
        );
        assert_eq!(*(phoneme.representation()), "aɪ̯");
    }

    #[test]
    fn parsing_tone_letters() {
        let (phoneme, warnings) = Phoneme::from("a˧˥");
        assert!(warnings.is_empty());
        assert!(
            phoneme
                .features()
                .contains(&Feature::Vowel(VowelFeature::Height(Height::Open)))
        );
        assert!(
            phoneme
                .features()
                .contains(&Feature::Vowel(VowelFeature::Depth(Depth::Front)))
        );
        assert!(phoneme.tone_letters().contains(&Tone::Mid));
        assert!(
            phoneme
                .tone_letters()
                .contains(&Tone::ExtraHigh)
        );
        assert_eq!(*(phoneme.representation()), "a˧˥");
    }

    #[test]
    fn parsing_prenasalized_double_articulation() {
        let (phoneme, warnings) = Phoneme::from("ŋ͡mg͡b");
        assert!(warnings.is_empty());
        assert!(phoneme.modifiers().contains(&Modifier::PreNasalized));
        assert!(phoneme.features().contains(&Feature::Consonant(ConsonantFeature::DoubleArticulation(Place::Bilabial, Place::Velar))));
        assert!(phoneme.features().contains(&Feature::Consonant(ConsonantFeature::Manner(Manner::Stop))));
    }

    #[test]
    fn name_simple() {
        let (phoneme, warnings) = Phoneme::from("t");
        assert!(warnings.is_empty());
        assert_eq!(phoneme.name(), "voiceless alveolar stop");
    }

    #[test]
    fn name_with_modifiers() {
        let (phoneme, warnings) = Phoneme::from("tʰ");
        assert!(warnings.is_empty());
        assert_eq!(phoneme.name(), "aspirated voiceless alveolar stop");
    }

    #[test]
    fn name_with_suffix() {
        let (phoneme, warnings) = Phoneme::from("á");
        assert!(warnings.is_empty());
        assert_eq!(phoneme.name(), "open front vowel with high tone");
    }

    #[test]
    fn name_voiceless() {
        let (phoneme, warnings) = Phoneme::from("d̥");
        assert!(warnings.is_empty());
        assert_eq!(phoneme.name(), "voiceless alveolar stop");
    }

    #[test]
    fn name_tone_letters() {
        let (phoneme, warnings) = Phoneme::from("a˧˥");
        assert!(warnings.is_empty());
        assert_eq!(phoneme.name(), "open front vowel with tone pattern 35");
    }

    #[test]
    fn name_prenasalized_double_articulation() {
        let (phoneme, warnings) = Phoneme::from("ŋ͡mg͡b");
        assert!(warnings.is_empty());
        assert_eq!(phoneme.name(), "pre-nasalized voiced labial-velar stop");
    }
}
