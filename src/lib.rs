use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Place {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Alveolopalatal,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Manner {
    Nasal,
    Stop,
    Affricate,
    Fricative,
    TrillFlap,
    LateralApproximant,
    Approximant,
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Depth {
    Front,
    NearFront,
    Central,
    NearBack,
    Back,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConsonantFeature {
    Place(Place),
    Manner(Manner),
    Voiced,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VowelFeature {
    Height(Height),
    Depth(Depth),
    Rounded,
}

pub enum Feature {
    Consonant(ConsonantFeature),
    Vowel(VowelFeature),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Modifier {
    NonSyllabic,
    Syllabic,
    Aspirated,
    InaudibleRelease,
    NasalRelease,
    PreNasalized,
    PostStopped,
    DentalFricativeRelease,
    LateralRelease,
    VelarFricativeRelease,
    SchwaRelease,
    Voiceless,
    Voiced,
    Breathy,
    Creaky,
    Dental,
    Linguolabial,
    Apical,
    Laminal,
    Advanced,
    Retracted,
    Centralized,
    MidCentralized,
    Raised,
    Lowered,
    OverRounded,
    UnderRounded,
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
    AdvancedTongueRoot,
    RetractedTongueRoot,
    Nasalized,
    Rhotacized,
    Long,
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
    HalfLong,
    ExtraShort,
}

static PHONEMES: phf::Map<&'static str, &'static [Feature]> = phf::phf_map! {
    
};

struct Phoneme {
    features: HashSet<Feature>,
    modifiers: HashSet<Modifier>,
}
