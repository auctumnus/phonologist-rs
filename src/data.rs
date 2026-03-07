//! Underlying data for parsing IPA.
//!
//! None of this is guaranteed to be stable across versions, and some of it may be incomplete or inaccurate. Use with caution.

/// All phonemes with their features. This list is not guaranteed to be stable across versions.
pub static PHONEMES: &[(&str, &[crate::Feature])] = {
    use crate::feature::Airstream::*;
    use crate::feature::FricativeKind::*;
    use crate::feature::RearArticulation;
    use crate::feature::Airstream;
    use crate::ConsonantFeature::*;
    use crate::Depth::*;
    use crate::Feature::*;
    use crate::Height::*;
    use crate::Manner::*;
    use crate::Place::*;
    use crate::VowelFeature::*;
    

    // must be ordered longest to shortest, otherwise the longest match will be missed
    &[
        // double articulations with overtie
        (
            "k͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "g͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɠ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "d͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɗ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɖ͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ᶑ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "c͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɟ͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʄ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɢ͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʛ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // 2-char affricates with over-tie
        (
            "p͡ɸ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "b͡β",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "p͡f",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "b͡v",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "t̪͡θ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "t͡θ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "d͡ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "d̪͡ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡s",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "d͡z",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡ɬ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
            ],
        ),
        (
            "d͡ɮ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡ʃ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "d͡ʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "d͡ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȶ͡ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "ȡ͡ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͡ʂ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "ɖ͡ʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͡ꞎ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ɖ͡𝼅",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɖ͡ɭ˔",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        (
            "c͡ç",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ɟ͡ʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡x",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "g͡ɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡χ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ɢ͡ʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // epiglottal affricates with over-tie
        (
            "ʡ͡ʜ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ʡ͡ʢ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // glottal affricate with over-tie
        (
            "ʔ͡h",
            &[
                Consonant(Place(Glottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        // palatal lateral affricates with over-tie
        (
            "c͡𝼆",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ɟ͡ʎ̝",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        // velar lateral affricates with over-tie
        (
            "k͡𝼄",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ɡ͡ʟ̝",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with over-tie — velar accompaniment
        (
            "k͡ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with over-tie — velar accompaniment (alternate transcriptions)
        (
            "k͡ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͡ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with over-tie — uvular accompaniment
        (
            "q͡ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with over-tie — uvular accompaniment (alternate transcriptions)
        (
            "q͡ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͡ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        // 2-char affricates with under-tie
        (
            "p͜ɸ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "b͜β",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "p͜f",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "b͜v",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜θ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "t̪͜θ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "d͜ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "d̪͜ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜s",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "d͜z",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜ʃ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "d͜ʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "d͜ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȶ͜ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "ȡ͜ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͜ʂ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "ɖ͜ʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "c͜ç",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ɟ͜ʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜x",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "g͜ɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜χ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ɢ͜ʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // epiglottal affricates with under-tie
        (
            "ʡ͜ʜ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ʡ͜ʢ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // glottal affricate with under-tie
        (
            "ʔ͜h",
            &[
                Consonant(Place(Glottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        // palatal lateral affricates with under-tie
        (
            "c͜𝼆",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ɟ͜ʎ̝",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        // velar lateral affricates with under-tie
        (
            "k͜𝼄",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ɡ͜ʟ̝",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with under-tie — velar accompaniment
        (
            "k͜ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with under-tie — velar accompaniment (alternate transcriptions)
        (
            "k͜ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ͜ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with under-tie — uvular accompaniment
        (
            "q͜ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        // click consonants with under-tie — uvular accompaniment (alternate transcriptions)
        (
            "q͜ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ͜ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        // double articulations with undertie
        (
            "k͜p",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "g͜b",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɠ͜ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜p",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "d͜b",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɗ͜ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͜p",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɖ͜b",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ᶑ͜ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "c͜p",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɟ͜b",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʄ͜ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜p",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɢ͜b",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʛ͜ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // double articulations
        (
            "kp",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "gb",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɠɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "tp",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "db",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɗɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈp",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɖb",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ᶑɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "cp",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɟb",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʄɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "qp",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɢb",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʛɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // 2-char affricates
        (
            "pɸ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "bβ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "pf",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "bv",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "tθ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "t̪θ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "d̪ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ts",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "dz",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "tʃ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "dʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "tɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "dʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȶɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "ȡʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈʂ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Sibilant))),
            ],
        ),
        (
            "ɖʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "cç",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ɟʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "kx",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "gɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "qχ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ɢʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // epiglottal affricates (no tie bar)
        (
            "ʡʜ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        (
            "ʡʢ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // glottal affricate (no tie bar)
        (
            "ʔh",
            &[
                Consonant(Place(Glottal)),
                Consonant(Manner(Affricate(NonSibilant(Median)))),
            ],
        ),
        // click consonants (no tie bar) — velar accompaniment
        (
            "kʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "kǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "kǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "kǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "kǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // click consonants (no tie bar) — uvular accompaniment
        (
            "qʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "qǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "qǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "qǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "qǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        // click consonants (no tie bar) — velar accompaniment (alternate transcriptions)
        (
            "kʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "k𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡ𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "kʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "kʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ɡʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // click consonants (no tie bar) — uvular accompaniment (alternate transcriptions)
        (
            "qʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "q𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢ𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "qʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "qʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
            ],
        ),
        (
            "ɢʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Uvular))),
                Consonant(Voiced),
            ],
        ),
        // bilabial
        (
            "m",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        ("p", &[Consonant(Place(Bilabial)), Consonant(Manner(Stop))]),
        (
            "b",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɸ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "β",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʙ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Trill)),
                Consonant(Voiced),
            ],
        ),
        (
            "ⱱ̟",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Flap(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɓ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // labiodental
        (
            "ɱ",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "f",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "v",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʋ",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ⱱ",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Flap(Median))),
                Consonant(Voiced),
            ],
        ),
        // dental
        ("t̪", &[Consonant(Place(Dental)), Consonant(Manner(Stop))]),
        (
            "d̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "θ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "s̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Fricative(Sibilant))),
            ],
        ),
        (
            "z̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Fricative(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ð̞",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Approximant(Median))),
            ],
        ),
        (
            "l̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɗ̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // linguolabial, coronal notation
        (
            "n̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "t̼",
            &[Consonant(Place(Linguolabial)), Consonant(Manner(Stop))],
        ),
        (
            "d̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "s̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Fricative(Sibilant))),
            ],
        ),
        (
            "z̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Fricative(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "θ̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ð̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "r̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Trill)),
                Consonant(Voiced),
            ],
        ),
        // linguolabial, labial notation
        (
            "m̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "p̼",
            &[Consonant(Place(Linguolabial)), Consonant(Manner(Stop))],
        ),
        (
            "b̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "f̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "v̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʙ̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Trill)),
                Consonant(Voiced),
            ],
        ),
        // alveolar
        (
            "n",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        ("t", &[Consonant(Place(Alveolar)), Consonant(Manner(Stop))]),
        (
            "d",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "s",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Fricative(Sibilant))),
            ],
        ),
        (
            "z",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Fricative(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʃ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Fricative(Sibilant))),
            ],
        ),
        (
            "ʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Fricative(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɬ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ɮ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        (
            "r",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Trill)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɾ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Flap(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɹ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɺ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Flap(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "l",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɫ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // alveolopalatal
        (
            "ȶ",
            &[Consonant(Place(Alveolopalatal)), Consonant(Manner(Stop))],
        ),
        (
            "ȡ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ȵ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Fricative(Sibilant))),
            ],
        ),
        (
            "ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Fricative(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȴ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        // retroflex
        ("ʈ", &[Consonant(Place(Retroflex)), Consonant(Manner(Stop))]),
        (
            "ɖ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʂ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Fricative(Sibilant))),
            ],
        ),
        (
            "ʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Fricative(Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɽ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Flap(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɻ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɭ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "ꞎ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ᶑ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // palatal
        ("c", &[Consonant(Place(Palatal)), Consonant(Manner(Stop))]),
        (
            "ɟ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɲ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "ç",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʎ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "j",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɥ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "𝼆",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ʎ̝",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʄ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // velar
        ("k", &[Consonant(Place(Velar)), Consonant(Manner(Stop))]),
        (
            "g",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "x",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "w",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʍ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Approximant(Median))),
            ],
        ),
        (
            "ɰ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Approximant(Median))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʟ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Approximant(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "𝼄",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
            ],
        ),
        (
            "ʟ̝",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Fricative(NonSibilant(Lateral)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʟ̆",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Flap(Lateral))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɠ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // uvular
        ("q", &[Consonant(Place(Uvular)), Consonant(Manner(Stop))]),
        (
            "ɢ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Nasal)),
                Consonant(Voiced),
            ],
        ),
        (
            "χ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʀ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Trill)),
                Consonant(Voiced),
            ],
        ),
        (
            "ʛ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        // pharyngeal
        (
            "ħ",
            &[
                Consonant(Place(Pharyngeal)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ʕ",
            &[
                Consonant(Place(Pharyngeal)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // epiglottal
        (
            "ʡ",
            &[Consonant(Place(Epiglottal)), Consonant(Manner(Stop))],
        ),
        (
            "ʜ",
            &[Consonant(Place(Epiglottal)), Consonant(Manner(Trill))],
        ),
        (
            "ʢ",
            &[
                Consonant(Place(Epiglottal)),
                Consonant(Manner(Trill)),
                Consonant(Voiced),
            ],
        ),
        // glottal
        ("ʔ", &[Consonant(Place(Glottal)), Consonant(Manner(Stop))]),
        (
            "h",
            &[
                Consonant(Place(Glottal)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
            ],
        ),
        (
            "ɦ",
            &[
                Consonant(Place(Glottal)),
                Consonant(Manner(Fricative(NonSibilant(Median)))),
                Consonant(Voiced),
            ],
        ),
        // clicks (bare symbols, default to velar rear articulation)
        (
            "ʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
            ],
        ),
        (
            "𝼊",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        // alternate click transcriptions
        (
            "ʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        // click consonants with superscript prefix — velar accompaniment
        (
            "ᵏʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍʘ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍǀ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍǃ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍǂ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍǁ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Lateral, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍʇ",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏ𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍ𝼋",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍʗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        (
            "ᵏʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
            ],
        ),
        (
            "ᵍʖ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Click(Airstream::Median, RearArticulation::Velar))),
                Consonant(Voiced),
            ],
        ),
        // close vowels
        ("i", &[Vowel(Height(Close)), Vowel(Depth(Front))]),
        (
            "y",
            &[Vowel(Height(Close)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ɨ", &[Vowel(Height(Close)), Vowel(Depth(Central))]),
        (
            "ʉ",
            &[Vowel(Height(Close)), Vowel(Depth(Central)), Vowel(Rounded)],
        ),
        ("ɯ", &[Vowel(Height(Close)), Vowel(Depth(Back))]),
        (
            "u",
            &[Vowel(Height(Close)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
        // near-close vowels
        ("ɪ", &[Vowel(Height(NearClose)), Vowel(Depth(NearFront))]),
        (
            "ʏ",
            &[
                Vowel(Height(NearClose)),
                Vowel(Depth(NearFront)),
                Vowel(Rounded),
            ],
        ),
        (
            "ʊ",
            &[
                Vowel(Height(NearClose)),
                Vowel(Depth(NearBack)),
                Vowel(Rounded),
            ],
        ),
        // close-mid vowels
        ("e", &[Vowel(Height(CloseMid)), Vowel(Depth(Front))]),
        (
            "ø",
            &[Vowel(Height(CloseMid)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ɘ", &[Vowel(Height(CloseMid)), Vowel(Depth(Central))]),
        (
            "ɵ",
            &[
                Vowel(Height(CloseMid)),
                Vowel(Depth(Central)),
                Vowel(Rounded),
            ],
        ),
        ("ɤ", &[Vowel(Height(CloseMid)), Vowel(Depth(Back))]),
        (
            "o",
            &[Vowel(Height(CloseMid)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
        // mid vowels
        ("ə", &[Vowel(Height(Mid)), Vowel(Depth(Central))]),
        // open-mid vowels
        ("ɛ", &[Vowel(Height(OpenMid)), Vowel(Depth(Front))]),
        (
            "œ",
            &[Vowel(Height(OpenMid)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ɜ", &[Vowel(Height(OpenMid)), Vowel(Depth(Central))]),
        (
            "ɞ",
            &[
                Vowel(Height(OpenMid)),
                Vowel(Depth(Central)),
                Vowel(Rounded),
            ],
        ),
        ("ʌ", &[Vowel(Height(OpenMid)), Vowel(Depth(Back))]),
        (
            "ɔ",
            &[Vowel(Height(OpenMid)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
        // near-open vowels
        ("æ", &[Vowel(Height(NearOpen)), Vowel(Depth(Front))]),
        ("ɐ", &[Vowel(Height(NearOpen)), Vowel(Depth(Central))]),
        // open vowels
        ("a", &[Vowel(Height(Open)), Vowel(Depth(Front))]),
        (
            "ɶ",
            &[Vowel(Height(Open)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ä", &[Vowel(Height(Open)), Vowel(Depth(Central))]),
        ("ɑ", &[Vowel(Height(Open)), Vowel(Depth(Back))]),
        (
            "ɒ",
            &[Vowel(Height(Open)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
    ]
};

/// See <https://en.wikipedia.org/wiki/Diphthong> / <https://en.wikipedia.org/wiki/Triphthong>.
/// This list is not guaranteed to be stable across versions.
pub static POLYPHTHONG_COMPONENTS: &[(&str, &[crate::Feature])] = {
    use crate::Depth::*;
    use crate::Feature::*;
    use crate::Height::*;
    use crate::VowelFeature::*;

    &[
        // close vowels
        ("i̯", &[Vowel(Height(Close)), Vowel(Depth(Front))]),
        (
            "y̯",
            &[Vowel(Height(Close)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ɨ̯", &[Vowel(Height(Close)), Vowel(Depth(Central))]),
        (
            "ʉ̯",
            &[Vowel(Height(Close)), Vowel(Depth(Central)), Vowel(Rounded)],
        ),
        ("ɯ̯", &[Vowel(Height(Close)), Vowel(Depth(Back))]),
        (
            "u̯",
            &[Vowel(Height(Close)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
        // near-close vowels
        ("ɪ̯", &[Vowel(Height(NearClose)), Vowel(Depth(NearFront))]),
        (
            "ʏ̯",
            &[
                Vowel(Height(NearClose)),
                Vowel(Depth(NearFront)),
                Vowel(Rounded),
            ],
        ),
        (
            "ʊ̯",
            &[
                Vowel(Height(NearClose)),
                Vowel(Depth(NearBack)),
                Vowel(Rounded),
            ],
        ),
        // close-mid vowels
        ("e̯", &[Vowel(Height(CloseMid)), Vowel(Depth(Front))]),
        (
            "ø̯",
            &[Vowel(Height(CloseMid)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ɘ̯", &[Vowel(Height(CloseMid)), Vowel(Depth(Central))]),
        (
            "ɵ̯",
            &[
                Vowel(Height(CloseMid)),
                Vowel(Depth(Central)),
                Vowel(Rounded),
            ],
        ),
        ("ɤ̯", &[Vowel(Height(CloseMid)), Vowel(Depth(Back))]),
        (
            "o̯",
            &[Vowel(Height(CloseMid)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
        // mid vowels
        ("ə̯", &[Vowel(Height(Mid)), Vowel(Depth(Central))]),
        // open-mid vowels
        ("ɛ̯", &[Vowel(Height(OpenMid)), Vowel(Depth(Front))]),
        (
            "œ̯",
            &[Vowel(Height(OpenMid)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ɜ̯", &[Vowel(Height(OpenMid)), Vowel(Depth(Central))]),
        (
            "ɞ̯",
            &[
                Vowel(Height(OpenMid)),
                Vowel(Depth(Central)),
                Vowel(Rounded),
            ],
        ),
        ("ʌ̯", &[Vowel(Height(OpenMid)), Vowel(Depth(Back))]),
        (
            "ɔ̯",
            &[Vowel(Height(OpenMid)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
        // near-open vowels
        ("æ̯", &[Vowel(Height(NearOpen)), Vowel(Depth(Front))]),
        ("ɐ̯", &[Vowel(Height(NearOpen)), Vowel(Depth(Central))]),
        // open vowels
        ("a̯", &[Vowel(Height(Open)), Vowel(Depth(Front))]),
        (
            "ɶ̯",
            &[Vowel(Height(Open)), Vowel(Depth(Front)), Vowel(Rounded)],
        ),
        ("ä̯", &[Vowel(Height(Open)), Vowel(Depth(Central))]),
        ("ɑ̯", &[Vowel(Height(Open)), Vowel(Depth(Back))]),
        (
            "ɒ̯",
            &[Vowel(Height(Open)), Vowel(Depth(Back)), Vowel(Rounded)],
        ),
    ]
};

pub static PRENASALIZED_STOPS: &[(&str, &[crate::Feature])] = {
    use crate::ConsonantFeature::*;
    use crate::Feature::*;
    use crate::Manner::*;
    use crate::Place::*;

    &[
        (
            "ŋ͡mk͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ŋ͡mg͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡mɠ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "n͡mt͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "n͡md͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n͡mɗ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̪͡mt̪͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Dental)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "n̪͡md̪͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Dental)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̪͡mɗ̪͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Dental)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼͡mt̼͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Linguolabial)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "n̼͡md̼͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Linguolabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼͡mɗ̼͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Linguolabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ͡mʈ͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɳ͡mɖ͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ͡mᶑ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɲ͡mɟ͡c",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɲ͡mɟ͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɲ͡mʄ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡mq͡p",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
            ],
        ),
        (
            "ɴ͡mɢ͡b",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡mʛ͡ɓ",
            &[
                Consonant(DoubleArticulation(Bilabial, Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("m͡p", &[Consonant(Place(Bilabial)), Consonant(Manner(Stop))]),
        (
            "m͡b",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "m͡ɓ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("n͡t", &[Consonant(Place(Alveolar)), Consonant(Manner(Stop))]),
        (
            "n͡d",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n͡ɗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("n̪͡t̪", &[Consonant(Place(Dental)), Consonant(Manner(Stop))]),
        (
            "n̪͡d̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̪͡ɗ̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼͡t̼",
            &[Consonant(Place(Linguolabial)), Consonant(Manner(Stop))],
        ),
        (
            "n̼͡d̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼͡ɗ̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ͡t",
            &[Consonant(Place(Retroflex)), Consonant(Manner(Stop))],
        ),
        (
            "ɳ͡ɖ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ͡ᶑ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ɲ͡c", &[Consonant(Place(Palatal)), Consonant(Manner(Stop))]),
        (
            "ɲ͡ɟ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɲ͡ʄ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ŋ͡k", &[Consonant(Place(Velar)), Consonant(Manner(Stop))]),
        (
            "ŋ͡g",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͡ɠ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ɴ͡q", &[Consonant(Place(Uvular)), Consonant(Manner(Stop))]),
        (
            "ɴ͡ɢ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͡ʛ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("m͜p", &[Consonant(Place(Bilabial)), Consonant(Manner(Stop))]),
        (
            "m͜b",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "m͜ɓ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("n͜t", &[Consonant(Place(Alveolar)), Consonant(Manner(Stop))]),
        (
            "n͜d",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n͜ɗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("n̪͜t̪", &[Consonant(Place(Dental)), Consonant(Manner(Stop))]),
        (
            "n̪͜d̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̪͜ɗ̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼͜t̼",
            &[Consonant(Place(Linguolabial)), Consonant(Manner(Stop))],
        ),
        (
            "n̼͜d̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼͜ɗ̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ͜t",
            &[Consonant(Place(Retroflex)), Consonant(Manner(Stop))],
        ),
        (
            "ɳ͜ɖ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳ͜ᶑ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ɲ͜c", &[Consonant(Place(Palatal)), Consonant(Manner(Stop))]),
        (
            "ɲ͜ɟ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɲ͜ʄ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ŋ͜k", &[Consonant(Place(Velar)), Consonant(Manner(Stop))]),
        (
            "ŋ͜g",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋ͜ɠ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ɴ͜q", &[Consonant(Place(Uvular)), Consonant(Manner(Stop))]),
        (
            "ɴ͜ɢ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴ͜ʛ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("mp", &[Consonant(Place(Bilabial)), Consonant(Manner(Stop))]),
        (
            "mb",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "mɓ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("nt", &[Consonant(Place(Alveolar)), Consonant(Manner(Stop))]),
        (
            "nd",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "nɗ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("n̪t̪", &[Consonant(Place(Dental)), Consonant(Manner(Stop))]),
        (
            "n̪d̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̪ɗ̪",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼t̼",
            &[Consonant(Place(Linguolabial)), Consonant(Manner(Stop))],
        ),
        (
            "n̼d̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "n̼ɗ̼",
            &[
                Consonant(Place(Linguolabial)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳt",
            &[Consonant(Place(Retroflex)), Consonant(Manner(Stop))],
        ),
        (
            "ɳɖ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɳᶑ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ɲc", &[Consonant(Place(Palatal)), Consonant(Manner(Stop))]),
        (
            "ɲɟ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɲʄ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ŋk", &[Consonant(Place(Velar)), Consonant(Manner(Stop))]),
        (
            "ŋg",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ŋɠ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
        ("ɴq", &[Consonant(Place(Uvular)), Consonant(Manner(Stop))]),
        (
            "ɴɢ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Stop)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɴʛ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Implosive)),
                Consonant(Voiced),
            ],
        ),
    ]
};

pub static PRENASALIZED_STOP_CONFUSABLE: phf::Set<&str> = phf::phf_set! {
        "m͡",
        "n͡",
        "n̪͡",
        "n̼͡",
        "ɳ͡",
        "ɲ͡",
        "ŋ͡",
        "ɴ͡",
        "m͜",
        "n͜",
        "ɳ͜",
        "ɲ͜",
        "ŋ͜",
        "ɴ͜",
};

/// Modifiers which appear before the base character. This list is not guaranteed to be stable across versions.
pub static PREFIX_MODIFIERS: &[(&str, crate::Modifier)] = {
    use crate::Modifier::*;

    &[
        ("ᶯ͡ᵐ", PreNasalized),
        ("ᵑ͡ᵐ", PreNasalized),
        ("ⁿ͡ᵐ", PreNasalized),
        ("m͡", PreNasalized),
        ("n͡", PreNasalized),
        ("n̪͡", PreNasalized),
        ("n̼͡", PreNasalized),
        ("ɳ͡", PreNasalized),
        ("ɲ͡", PreNasalized),
        ("ŋ͡", PreNasalized),
        ("ɴ͡", PreNasalized),
        ("m͜", PreNasalized),
        ("n͜", PreNasalized),
        ("n̪͜", PreNasalized),
        ("n̼͜", PreNasalized),
        ("ɳ͜", PreNasalized),
        ("ɲ͜", PreNasalized),
        ("ŋ͜", PreNasalized),
        ("ɴ͜", PreNasalized),
        ("ᵐ", PreNasalized),
        ("ⁿ", PreNasalized),
        ("ᶯ", PreNasalized),
        ("ᶮ", PreNasalized),
        ("ᵑ", PreNasalized),
        ("ᶰ", PreNasalized),
        ("ᵇ", PreStopped),
        ("ᵈ", PreStopped),
        ("ᶡ", PreStopped),
        ("ᶢ", PreStopped),
        ("ᵖ", PreStopped),
        ("ᵗ", PreStopped),
        ("ᶜ", PreStopped),
        ("ᵏ", PreStopped),
        ("ʰ", PreAspirated),
        ("ˣ", VelarFricativeOnset),
    ]
};

/// Modifiers which appear after the base character. This list is not guaranteed to be stable across versions.
pub static POSTFIX_MODIFIERS: &[(&str, crate::Modifier)] = {
    use crate::Modifier::*;

    &[
        ("̯", NonSyllabic),
        ("̑", NonSyllabic),
        ("̩", Syllabic),
        ("̍", Syllabic),
        ("ʰ", Aspirated),
        ("̚", Release(crate::feature::Release::None)),
        ("ⁿ", Release(crate::feature::Release::Nasal)),
        ("ᵇ", PostStopped),
        ("ᵈ", PostStopped),
        ("ᶡ", PostStopped),
        ("ᶢ", PostStopped),
        ("ᵖ", PostStopped),
        ("ᵗ", PostStopped),
        ("ᶜ", PostStopped),
        ("ᵏ", PostStopped),
        ("ᶿ", Release(crate::feature::Release::DentalFricative)),
        ("ˡ", Release(crate::feature::Release::Lateral)),
        ("ˣ", Release(crate::feature::Release::VelarFricative)),
        ("ᵊ", Release(crate::feature::Release::Schwa)),
        ("̥", Voice(crate::feature::Voice::Voiceless)),
        ("̊", Voice(crate::feature::Voice::Voiceless)),
        ("̬", Voice(crate::feature::Voice::Voiced)),
        ("̤", Voice(crate::feature::Voice::Breathy)),
        ("̰", Voice(crate::feature::Voice::Creaky)),
        ("̺", Apical),
        ("̻", Laminal),
        (
            "̟",
            RelativeArticulation(crate::feature::RelativeArticulation::Advanced),
        ),
        (
            "̠",
            RelativeArticulation(crate::feature::RelativeArticulation::Retracted),
        ),
        (
            "˖",
            RelativeArticulation(crate::feature::RelativeArticulation::Advanced),
        ),
        (
            "˗",
            RelativeArticulation(crate::feature::RelativeArticulation::Retracted),
        ),
        (
            "̈",
            RelativeArticulation(crate::feature::RelativeArticulation::Centralized),
        ),
        (
            "̽",
            RelativeArticulation(crate::feature::RelativeArticulation::MidCentralized),
        ),
        (
            "̝",
            RelativeArticulation(crate::feature::RelativeArticulation::Raised),
        ),
        (
            "˔",
            RelativeArticulation(crate::feature::RelativeArticulation::Raised),
        ),
        (
            "̞",
            RelativeArticulation(crate::feature::RelativeArticulation::Lowered),
        ),
        (
            "˕",
            RelativeArticulation(crate::feature::RelativeArticulation::Lowered),
        ),
        (
            "̹",
            RelativeArticulation(crate::feature::RelativeArticulation::OverRounded),
        ),
        (
            "͗",
            RelativeArticulation(crate::feature::RelativeArticulation::OverRounded),
        ),
        (
            "̜",
            RelativeArticulation(crate::feature::RelativeArticulation::UnderRounded),
        ),
        (
            "͑",
            RelativeArticulation(crate::feature::RelativeArticulation::UnderRounded),
        ),
        (
            "ʷ",
            SecondaryArticulation(crate::feature::SecondaryArticulation::Labialized),
        ),
        (
            "ʲ",
            SecondaryArticulation(crate::feature::SecondaryArticulation::Palatalized),
        ),
        (
            "ˠ",
            SecondaryArticulation(crate::feature::SecondaryArticulation::Velarized),
        ),
        (
            "̴",
            SecondaryArticulation(crate::feature::SecondaryArticulation::Velarized),
        ),
        (
            "ˤ",
            SecondaryArticulation(crate::feature::SecondaryArticulation::Pharyngealized),
        ),
        (
            "𐞴",
            SecondaryArticulation(crate::feature::SecondaryArticulation::Epiglottalized),
        ),
        ("̘", TongueRoot(crate::feature::TongueRoot::Advanced)),
        ("̙", TongueRoot(crate::feature::TongueRoot::Retracted)),
        ("̃", Nasalized),
        ("˞", Rhotacized),
        ("ʴ", Rhotacized),
        ("ʵ", Rhotacized),
        ("ʱ", Voice(crate::feature::Voice::Breathy)),
        ("ː", Length(crate::feature::Length::Long)),
        (":", Length(crate::feature::Length::Long)),
        ("̋", Tone(crate::feature::Tone::ExtraHigh)),
        ("́", Tone(crate::feature::Tone::High)),
        ("̄", Tone(crate::feature::Tone::Mid)),
        ("̀", Tone(crate::feature::Tone::Low)),
        ("̏", Tone(crate::feature::Tone::ExtraLow)),
        ("̌", Tone(crate::feature::Tone::Rising)),
        ("̂", Tone(crate::feature::Tone::Falling)),
        ("᷄", Tone(crate::feature::Tone::HighRising)),
        ("᷅", Tone(crate::feature::Tone::LowRising)),
        ("᷈", Tone(crate::feature::Tone::RisingFalling)),
        ("ˑ", Length(crate::feature::Length::HalfLong)),
        ("̆", Length(crate::feature::Length::ExtraShort)),
        ("ʼ", Ejective),
        ("\u{2019}", Ejective), // U+2019 right single quotation mark, sometimes used for ejective,
        ("͡χ", Release(crate::feature::Release::UvularFricative))
    ]
};

/// See <https://en.wikipedia.org/wiki/Tone_letter>.
pub static TONE_LETTERS: &[(&str, crate::feature::Tone)] = {
    &[
        // stem to the right
        ("˥", crate::feature::Tone::ExtraHigh),
        ("˦", crate::feature::Tone::High),
        ("˧", crate::feature::Tone::Mid),
        ("˨", crate::feature::Tone::Low),
        ("˩", crate::feature::Tone::ExtraLow),
        // stem to the left
        ("꜒", crate::feature::Tone::ExtraHigh),
        ("꜓", crate::feature::Tone::High),
        ("꜔", crate::feature::Tone::Mid),
        ("꜕", crate::feature::Tone::Low),
        ("꜖", crate::feature::Tone::ExtraLow),
        // dotted tone letters
        ("꜈", crate::feature::Tone::ExtraHigh),
        ("꜉", crate::feature::Tone::High),
        ("꜊", crate::feature::Tone::Mid),
        ("꜋", crate::feature::Tone::Low),
        ("꜌", crate::feature::Tone::ExtraLow),
        // dotted tone letters with stem to the left
        ("꜍", crate::feature::Tone::ExtraHigh),
        ("꜎", crate::feature::Tone::High),
        ("꜏", crate::feature::Tone::Mid),
        ("꜐", crate::feature::Tone::Low),
        ("꜑", crate::feature::Tone::ExtraLow),
        // numerals
        ("5", crate::feature::Tone::ExtraHigh),
        ("4", crate::feature::Tone::High),
        ("3", crate::feature::Tone::Mid),
        ("2", crate::feature::Tone::Low),
        ("1", crate::feature::Tone::ExtraLow),
        // superscript numerals
        ("⁵", crate::feature::Tone::ExtraHigh),
        ("⁴", crate::feature::Tone::High),
        ("³", crate::feature::Tone::Mid),
        ("²", crate::feature::Tone::Low),
        ("¹", crate::feature::Tone::ExtraLow),
    ]
};

pub static CLICKS: &[&str] = &["ʘ", "ǀ", "ǃ", "ǂ", "ǁ", "𝼊", "ʇ", "𝼋", "ʗ", "ʖ"];

pub static IMPLIED_MODIFIERS: phf::Map<&str, &[crate::Modifier]> = {
    use crate::Modifier::*;
    phf::phf_map! {
        "ɚ" | "ɝ" => &[Rhotacized],
        "ɫ" => &[SecondaryArticulation(crate::feature::SecondaryArticulation::Velarized)],
        "ŋ͡ʘ" | "ŋ͡ǀ" | "ŋ͡ǃ" | "ŋ͡ǂ" | "ŋ͡ǁ" |
        "ɴ͡ʘ" | "ɴ͡ǀ" | "ɴ͡ǃ" | "ɴ͡ǂ" | "ɴ͡ǁ" |
        "ŋ͜ʘ" | "ŋ͜ǀ" | "ŋ͜ǃ" | "ŋ͜ǂ" | "ŋ͜ǁ" |
        "ɴ͜ʘ" | "ɴ͜ǀ" | "ɴ͜ǃ" | "ɴ͜ǂ" | "ɴ͜ǁ" |
        "ŋʘ" | "ŋǀ" | "ŋǃ" | "ŋǂ" | "ŋǁ" |
        "ɴʘ" | "ɴǀ" | "ɴǃ" | "ɴǂ" | "ɴǁ" => &[Nasalized]
    }
};
