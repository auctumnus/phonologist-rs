//! Types for representing phonological features.

use std::collections::VecDeque;

macro_rules! display_for_static_str {
    ($ty:ident) => {
        impl $ty {
            /// Returns a human-friendly representation.
            pub fn name(&self) -> &'static str {
                self.into()
            }
        }
    };
}

/// The place of articulation of a consonant.
/// See <https://en.wikipedia.org/wiki/Place_of_articulation>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Place {
    /// See <https://en.wikipedia.org/wiki/Bilabial_consonant>.
    Bilabial,
    /// See <https://en.wikipedia.org/wiki/Labiodental_consonant>.
    Labiodental,
    /// See <https://en.wikipedia.org/wiki/Linguolabial_consonant>.
    Linguolabial,
    /// See <https://en.wikipedia.org/wiki/Dental_consonant>.
    Dental,
    /// See <https://en.wikipedia.org/wiki/Alveolar_consonant>.
    Alveolar,
    /// See <https://en.wikipedia.org/wiki/Post-alveolar_consonant>.
    PostAlveolar,
    /// See <https://en.wikipedia.org/wiki/Retroflex_consonant>.
    Alveolopalatal,
    /// See <https://en.wikipedia.org/wiki/Retroflex_consonant>.
    Retroflex,
    /// See <https://en.wikipedia.org/wiki/Palatal_consonant>.
    Palatal,
    /// See <https://en.wikipedia.org/wiki/Velar_consonant>.
    Velar,
    /// See <https://en.wikipedia.org/wiki/Uvular_consonant>.
    Uvular,
    /// See <https://en.wikipedia.org/wiki/Pharyngeal_consonant>.
    Pharyngeal,
    /// See <https://en.wikipedia.org/wiki/Epiglottal_consonant>.
    Epiglottal,
    /// See <https://en.wikipedia.org/wiki/Glottal_consonant>.
    Glottal,
}

/// Whether a consonant is lateral or not.
///
/// See <https://en.wikipedia.org/wiki/Lateral_consonant>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
pub enum Airstream {
    /// See <https://en.wikipedia.org/wiki/Median_consonant>.
    #[strum(serialize = "")]
    Median,
    #[strum(serialize = "lateral ")]
    Lateral,
}

/// The type of fricative involved in this sound.
///
/// See <https://en.wikipedia.org/wiki/Fricative>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FricativeKind {
    /// See <https://en.wikipedia.org/wiki/Sibilant>.
    Sibilant,
    NonSibilant(Airstream),
}

impl From<FricativeKind> for &'static str {
    fn from(value: FricativeKind) -> Self {
        match value {
            FricativeKind::Sibilant => "sibilant",
            FricativeKind::NonSibilant(Airstream::Median) => "fricative",
            FricativeKind::NonSibilant(Airstream::Lateral) => "lateral fricative",
        }
    }
}

impl<'a> From<&'a FricativeKind> for &'static str {
    fn from(value: &'a FricativeKind) -> Self {
        (*value).into()
    }
}

display_for_static_str!(FricativeKind);

/// The rear articulation of a click consonant.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
pub enum RearArticulation {
    #[strum(serialize = "")]
    Velar,
    #[strum(serialize = "uvular ")]
    Uvular,
}

/// The manner of articulation of a consonant.
///
/// See <https://en.wikipedia.org/wiki/Manner_of_articulation>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Manner {
    /// See <https://en.wikipedia.org/wiki/Nasal_consonant>.
    Nasal,
    /// See <https://en.wikipedia.org/wiki/Stop_consonant>.
    Stop,
    /// See <https://en.wikipedia.org/wiki/Affricate>.
    Affricate(FricativeKind),
    /// See <https://en.wikipedia.org/wiki/Fricative>.
    Fricative(FricativeKind),
    /// See <https://en.wikipedia.org/wiki/Trill_consonant>.
    Trill,
    /// See <https://en.wikipedia.org/wiki/Flap_consonant>.
    Flap(Airstream),
    /// See <https://en.wikipedia.org/wiki/Implosive_consonant>.
    Implosive,
    /// See <https://en.wikipedia.org/wiki/Approximant>.
    Approximant(Airstream),
    /// See <https://en.wikipedia.org/wiki/Click_consonant>.
    Click(RearArticulation),
}

impl From<Manner> for &'static str {
    fn from(value: Manner) -> Self {
        use Airstream::*;
        use FricativeKind::*;
        use Manner::*;
        use RearArticulation::*;
        match value {
            Nasal => "nasal",
            Stop => "stop",
            Trill => "trill",
            Implosive => "implosive",
            Approximant(Median) => "approximant",
            Approximant(Lateral) => "lateral approximant",
            Flap(Median) => "flap",
            Flap(Lateral) => "lateral flap",
            Click(Velar) => "click",
            Click(Uvular) => "uvular click",
            Affricate(kind) => match kind {
                Sibilant | NonSibilant(Median) => "affricate",
                NonSibilant(Lateral) => "lateral affricate",
            },
            Fricative(kind) => match kind {
                Sibilant | NonSibilant(Median) => "fricative",
                NonSibilant(Lateral) => "lateral fricative",
            },
        }
    }
}

impl<'a> From<&'a Manner> for &'static str {
    fn from(value: &'a Manner) -> Self {
        (*value).into()
    }
}

display_for_static_str!(Manner);

/// The height, or openness, of a vowel.
/// See <https://en.wikipedia.org/wiki/Vowel#Height>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

display_for_static_str!(Height);

/// The depth of a vowel.
/// See <https://en.wikipedia.org/wiki/Vowel#Backness>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Depth {
    Front,
    NearFront,
    Central,
    NearBack,
    Back,
}

display_for_static_str!(Depth);

/// Phonological features pertaining to consonants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConsonantFeature {
    Voiced,
    Place(Place),
    DoubleArticulation(Place, Place),
    Manner(Manner),
}

macro_rules! place_pairs {
    ($p1:expr, $p2:expr; $(($variant:ident, $name:expr)),* $(,)?) => {{
        macro_rules! inner {
            ($lhs_variant:ident, $lhs_name:expr) => {
                match $p2 {
                    $(crate::feature::Place::$variant => concat!($lhs_name, "-", $name),)*
                }
            };
        }
        match $p1 {
            $(crate::feature::Place::$variant => inner!($variant, $name),)*
        }
    }};
}

impl From<ConsonantFeature> for &'static str {
    fn from(value: ConsonantFeature) -> Self {
        use ConsonantFeature::*;

        match value {
            Voiced => "voiced",
            Place(p) => p.into(),
            Manner(m) => m.into(),
            DoubleArticulation(p1, p2) => place_pairs!(p1, p2;
                (Bilabial, "labial"),
                (Labiodental, "labiodental"),
                (Linguolabial, "linguolabial"),
                (Dental, "dental"),
                (Alveolar, "alveolar"),
                (PostAlveolar, "post-alveolar"),
                (Alveolopalatal, "alveolopalatal"),
                (Retroflex, "retroflex"),
                (Palatal, "palatal"),
                (Velar, "velar"),
                (Uvular, "uvular"),
                (Pharyngeal, "pharyngeal"),
                (Epiglottal, "epiglottal"),
                (Glottal, "glottal"),
            ),
        }
    }
}

impl<'a> From<&'a ConsonantFeature> for &'static str {
    fn from(value: &'a ConsonantFeature) -> Self {
        (*value).into()
    }
}

display_for_static_str!(ConsonantFeature);

/// Phonological features pertaining to vowels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VowelFeature {
    Height(Height),
    Depth(Depth),
    Rounded,
}

impl From<VowelFeature> for &'static str {
    fn from(value: VowelFeature) -> Self {
        match value {
            VowelFeature::Height(h) => h.into(),
            VowelFeature::Depth(d) => d.into(),
            VowelFeature::Rounded => "rounded",
        }
    }
}

impl<'a> From<&'a VowelFeature> for &'static str {
    fn from(value: &'a VowelFeature) -> Self {
        (*value).into()
    }
}

display_for_static_str!(VowelFeature);

/// Phonological features.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Feature {
    Consonant(ConsonantFeature),
    Vowel(VowelFeature),
}

impl From<Feature> for &'static str {
    fn from(value: Feature) -> Self {
        match value {
            Feature::Consonant(f) => f.into(),
            Feature::Vowel(f) => f.into(),
        }
    }
}

impl<'a> From<&'a Feature> for &'static str {
    fn from(value: &'a Feature) -> Self {
        (*value).into()
    }
}

display_for_static_str!(Feature);

impl Feature {
    /// Returns which `PhonemeClass` this feature is from.
    pub fn phoneme_class(&self) -> PhonemeClass {
        match self {
            Feature::Consonant(_) => PhonemeClass::Consonant,
            Feature::Vowel(_) => PhonemeClass::Vowel,
        }
    }
}

/// See <https://en.wikipedia.org/wiki/Voice_(phonetics)>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Voice {
    /// Occasionally, notation like /d̥/ may actually indicate a fortis/lenis distinction.
    Voiceless,
    /// See <https://en.wikipedia.org/wiki/Breathy_voice>.
    Breathy,
    /// See <https://en.wikipedia.org/wiki/Slack_voice>.
    Slack,
    /// Also called "modal voice".
    /// Occasionally, notation like /t̬/ may actually indicate a fortis/lenis distinction.
    Voiced,
    /// See <https://en.wikipedia.org/wiki/Stiff_voice>.
    Stiff,
    /// See <https://en.wikipedia.org/wiki/Creaky_voice>.
    Creaky,
    /// See <https://en.wikipedia.org/wiki/Glottalization_(phonetics)>.
    Glottalized,
    /// See <https://en.wikipedia.org/wiki/Faucalized_voice>.
    Faucalized,
    /// See <https://en.wikipedia.org/wiki/Strident_vowel>.
    Strident,
    /// See <https://en.wikipedia.org/wiki/Harsh_voice>.
    Ventricular,
}

display_for_static_str!(Voice);

/// See <https://en.wikipedia.org/wiki/Relative_articulation>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum RelativeArticulation {
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#Advanced_and_retracted>.
    Advanced,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#Advanced_and_retracted>.
    Retracted,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#Centralized_vowels>.
    Centralized,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#Mid-centralized_vowels>.
    MidCentralized,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#Raised_and_lowered>.
    Raised,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#Raised_and_lowered>.
    Lowered,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#More_and_less_rounded>.
    OverRounded,
    /// See <https://en.wikipedia.org/wiki/Relative_articulation#More_and_less_rounded>.
    UnderRounded,
}

display_for_static_str!(RelativeArticulation);

/// See <https://en.wikipedia.org/wiki/Secondary_articulation>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SecondaryArticulation {
    /// See <https://en.wikipedia.org/wiki/Labialization>.
    Labialized,
    /// See <https://en.wikipedia.org/wiki/Palatalization_(phonetics)>.
    Palatalized,
    /// See <https://en.wikipedia.org/wiki/Labio-palatalization>.
    LabioPalatalized,
    /// See <https://en.wikipedia.org/wiki/Velarization>.
    Velarized,
    /// See <https://en.wikipedia.org/wiki/Pharyngealization>.
    Pharyngealized,
    /// See <https://en.wikipedia.org/wiki/Epiglottal_consonant>.
    Epiglottalized,
}

display_for_static_str!(SecondaryArticulation);

/// See <https://en.wikipedia.org/wiki/Vowel_length>.
/// Length::Long also marks geminate consonants; see <https://en.wikipedia.org/wiki/Gemination>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Length {
    Long,
    HalfLong,
    Short,
    ExtraShort,
}

display_for_static_str!(Length);

/// See <https://en.wikipedia.org/wiki/Tone_(linguistics)>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case", prefix = "with ", suffix = " tone")]
pub enum Tone {
    ExtraHigh,
    High,
    Mid,
    Low,
    ExtraLow,
    Rising,
    Falling,
    HighRising,
    LowRising,
    RisingFalling,
}

impl Tone {
    pub fn as_number_str(self) -> &'static str {
        match self {
            Tone::ExtraHigh => "5",
            Tone::High => "4",
            Tone::Mid => "3",
            Tone::Low => "2",
            Tone::ExtraLow => "1",
            Tone::Rising => "24",
            Tone::Falling => "42",
            Tone::HighRising => "45",
            Tone::LowRising => "23",
            Tone::RisingFalling => "243",
        }
    }
}

display_for_static_str!(Tone);

/// The release of a consonant.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "lowercase", prefix = "with ", suffix = " release")]
pub enum Release {
    /// See <https://en.wikipedia.org/wiki/No_audible_release>.
    #[strum(serialize = "no audible")]
    None,
    /// Usually notates "excrescent schwas".
    Schwa,
    /// See <https://en.wikipedia.org/wiki/Lateral_release_(phonetics)>.
    Lateral,
    /// See <https://en.wikipedia.org/wiki/Nasal_release>.
    Nasal,
    #[strum(serialize = "dental fricative")]
    DentalFricative,
    /// As in <https://en.wikipedia.org/wiki/Lakota_language#Phonology>.
    #[strum(serialize = "velar fricative")]
    VelarFricative,
}

display_for_static_str!(Release);

/// See <https://en.wikipedia.org/wiki/Advanced_and_retracted_tongue_root>.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "lowercase", prefix = "with ", suffix = " tongue root")]
pub enum TongueRoot {
    Advanced,
    Retracted,
}

display_for_static_str!(TongueRoot);

/// Modifiers are usually represented as diacritics in IPA, but can also be tone letters or other markers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive] // such that we can add more later without breaking backwards compatibility
pub enum Modifier {
    /// See <https://en.wikipedia.org/wiki/Semivowel>.
    NonSyllabic,
    /// See <https://en.wikipedia.org/wiki/Syllabic_consonant>.
    Syllabic,
    /// See <https://en.wikipedia.org/wiki/Aspirated_consonant>.
    Aspirated,
    /// See <https://en.wikipedia.org/wiki/Aspirated_consonant#Preaspiration>.
    PreAspirated,
    /// See <https://en.wikipedia.org/wiki/Apical_consonant>.
    Apical,
    /// See <https://en.wikipedia.org/wiki/Laminal_consonant>.
    Laminal,
    Release(Release),
    /// See <https://en.wikipedia.org/wiki/Prenasalized_consonant>.
    PreNasalized,
    /// See <https://en.wikipedia.org/wiki/Nasal_consonant#Other_kinds_of_nasal_consonant>.
    PostStopped,
    /// See <https://en.wikipedia.org/wiki/Pre-stopped_consonant>.
    PreStopped,
    /// See <https://en.wikipedia.org/wiki/Affricate#Pre-affrication>.
    VelarFricativeOnset,
    Voice(Voice),
    SecondaryArticulation(SecondaryArticulation),
    /// See <https://en.wikipedia.org/wiki/Relative_articulation>.
    RelativeArticulation(RelativeArticulation),
    /// See <https://en.wikipedia.org/wiki/Advanced_and_retracted_tongue_root>.
    TongueRoot(TongueRoot),
    /// See <https://en.wikipedia.org/wiki/Nasalization>.
    Nasalized,
    /// See <https://en.wikipedia.org/wiki/R-colored_vowel>.
    Rhotacized,
    /// See <https://en.wikipedia.org/wiki/Tone_(linguistics)>.
    Tone(Tone),
    /// See <https://en.wikipedia.org/wiki/Vowel_length>.
    Length(Length),
    /// See <https://en.wikipedia.org/wiki/Ejective_consonant>.
    Ejective,
}

impl From<Modifier> for &'static str {
    fn from(value: Modifier) -> Self {
        use Modifier::*;
        match value {
            NonSyllabic => "non-syllabic",
            Syllabic => "syllabic",
            Aspirated => "aspirated",
            PreAspirated => "pre-aspirated",
            Apical => "apical",
            Laminal => "laminal",
            Release(v) => v.into(),
            PreNasalized => "pre-nasalized",
            PostStopped => "post-stopped",
            PreStopped => "pre-stopped",
            VelarFricativeOnset => "with velar fricative onset",
            Voice(v) => v.into(),
            SecondaryArticulation(v) => v.into(),
            RelativeArticulation(v) => v.into(),
            TongueRoot(v) => v.into(),
            Nasalized => "nasalized",
            Rhotacized => "rhotacized",
            Tone(v) => v.into(),
            Length(v) => v.into(),
            Ejective => "ejective",
        }
    }
}

impl<'a> From<&'a Modifier> for &'static str {
    fn from(value: &'a Modifier) -> Self {
        (*value).into()
    }
}

display_for_static_str!(Modifier);

impl Modifier {
    pub(crate) fn apply_modifier(&self, name: &mut VecDeque<&str>) {
        use Modifier::*;
        enum Affix {
            Prefix,
            Suffix,
        }
        let direction = match self {
            NonSyllabic
            | Syllabic
            | Aspirated
            | PreAspirated
            | PreNasalized
            | PreStopped
            | Apical
            | Laminal
            | SecondaryArticulation(_)
            | Nasalized
            | Rhotacized
            | Ejective
            | Length(_)
            | Voice(_)
            | RelativeArticulation(_)
            | PostStopped => Affix::Prefix,
            Release(_) | VelarFricativeOnset | Tone(_) | TongueRoot(_) => Affix::Suffix,
        };
        match direction {
            Affix::Prefix => name.push_front(self.into()),
            Affix::Suffix => name.push_back(self.into()),
        }
    }
}

/// The principal classes of speech sounds.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display, strum::IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum PhonemeClass {
    Consonant,
    Vowel,
}

display_for_static_str!(PhonemeClass);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_labial_velar() {
        let labial_velar = ConsonantFeature::DoubleArticulation(Place::Bilabial, Place::Velar);
        assert_eq!("labial-velar", Into::<&'static str>::into(labial_velar));
    }
}
