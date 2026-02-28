/// The place of articulation of a consonant.
/// See <https://en.wikipedia.org/wiki/Place_of_articulation>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    /// See <https://en.wikipedia.org/wiki/Glottal_consonant>.
    Glottal,
}

/// Whether a consonant is lateral or not.
///
/// See <https://en.wikipedia.org/wiki/Lateral_consonant>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Airstream {
    Median,
    Lateral,
}

/// Whether a consonant is sibilant or not.
/// See <https://en.wikipedia.org/wiki/Sibilant_consonant>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sibilancy {
    NonSibilant,
    Sibilant,
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
    Affricate(Airstream, Sibilancy),
    /// See <https://en.wikipedia.org/wiki/Lateral_affricate>.
    LateralAffricate,
    /// See <https://en.wikipedia.org/wiki/Fricative>.
    Fricative(Airstream, Sibilancy),
    /// See <https://en.wikipedia.org/wiki/Lateral_fricative>.
    LateralFricative,
    /// See <https://en.wikipedia.org/wiki/Trill_consonant>.
    Trill,
    /// See <https://en.wikipedia.org/wiki/Flap_consonant>.
    Flap,
    /// See <https://en.wikipedia.org/wiki/Approximant>.
    Approximant(Airstream),
    /// See <https://en.wikipedia.org/wiki/Click_consonant>.
    Click,
}

/// The height, or openness, of a vowel.
/// See <https://en.wikipedia.org/wiki/Vowel#Height>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

/// The depth of a vowel.
/// See <https://en.wikipedia.org/wiki/Vowel#Backness>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Depth {
    Front,
    NearFront,
    Central,
    NearBack,
    Back,
}

/// Phonological features pertaining to consonants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConsonantFeature {
    Place(Place),
    DoubleArticulation(Place, Place),
    Manner(Manner),
    Voiced,
}

/// Phonological features pertaining to vowels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VowelFeature {
    Height(Height),
    Depth(Depth),
    Rounded,
}

/// Phonological features.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Feature {
    Consonant(ConsonantFeature),
    Vowel(VowelFeature),
}

/// See <https://en.wikipedia.org/wiki/Voice_(phonetics)>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// See <https://en.wikipedia.org/wiki/Relative_articulation>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// See <https://en.wikipedia.org/wiki/Secondary_articulation>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// See <https://en.wikipedia.org/wiki/Vowel_length>.
/// Length::Long also marks geminate consonants; see <https://en.wikipedia.org/wiki/Gemination>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Length {
    Long,
    HalfLong,
    Short,
    ExtraShort,
}

/// See <https://en.wikipedia.org/wiki/Tone_(linguistics)>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// The release of a consonant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Release {
    /// See <https://en.wikipedia.org/wiki/No_audible_release>.
    None,
    /// Usually notates "excrescent schwas".
    Schwa,
    /// See <https://en.wikipedia.org/wiki/Lateral_release_(phonetics)>.
    Lateral,
    /// See <https://en.wikipedia.org/wiki/Nasal_release>.
    Nasal,
    DentalFricative,
    /// As in <https://en.wikipedia.org/wiki/Lakota_language#Phonology>.
    VelarFricative,
}

/// See <https://en.wikipedia.org/wiki/Advanced_and_retracted_tongue_root>.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TongueRoot {
    Advanced,
    Retracted,
}

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
    /// See <https://en.wikipedia.org/wiki/Dental_consonant>.
    Dental,
    /// See <https://en.wikipedia.org/wiki/Linguolabial_consonant>.
    Linguolabial,
    /// See <https://en.wikipedia.org/wiki/Apical_consonant>.
    Apical,
    /// See <https://en.wikipedia.org/wiki/Laminal_consonant>.
    Laminal,
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
}
