//! Underlying data for parsing IPA.
//!
//! None of this is guaranteed to be stable across versions, and some of it may be incomplete or inaccurate. Use with caution.

/// All phonemes with their features. This list is not guaranteed to be stable across versions.
pub static PHONEMES: &[(&str, &[crate::Feature])] = {
    use crate::ConsonantFeature::*;
    use crate::Depth::*;
    use crate::feature::Airstream::*;
    use crate::feature::Sibilancy::*;
    use crate::Feature::*;
    use crate::Height::*;
    use crate::Manner::*;
    use crate::Place::*;
    use crate::VowelFeature::*;

    // must be ordered longest to shortest, otherwise the longest match will be missed
    &[
        // 2-char affricates with over-tie
        (
            "p͡ɸ",
            &[Consonant(Place(Bilabial)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "b͡β",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "p͡f",
            &[Consonant(Place(Labiodental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "b͡v",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t̪͡θ",
            &[Consonant(Place(Dental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "t͡θ",
            &[Consonant(Place(Dental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "d͡ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "d̪͡ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡s",
            &[Consonant(Place(Alveolar)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "d͡z",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡ɬ",
            &[Consonant(Place(Alveolar)), Consonant(Manner(LateralAffricate))],
        ),
        (
            "d͡ɮ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(LateralAffricate)),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡ʃ",
            &[Consonant(Place(PostAlveolar)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "d͡ʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͡ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
            ],
        ),
        (
            "d͡ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȶ͡ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
            ],
        ),
        (
            "ȡ͡ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͡ʂ",
            &[Consonant(Place(Retroflex)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "ɖ͡ʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͡ꞎ",
            &[Consonant(Place(Retroflex)), Consonant(Manner(LateralAffricate))],
        ),
        (
            "ɖ͡𝼅",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(LateralAffricate)),
                Consonant(Voiced),
            ],
        ),
        (
            "ɖ͡ɭ˔",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(LateralAffricate)),
                Consonant(Voiced),
            ],
        ),
        (
            "c͡ç",
            &[Consonant(Place(Palatal)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "ɟ͡ʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͡x",
            &[Consonant(Place(Velar)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "g͡ɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͡χ",
            &[Consonant(Place(Uvular)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "ɢ͡ʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        // 2-char affricates with under-tie
        (
            "p͜ɸ",
            &[Consonant(Place(Bilabial)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "b͜β",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "p͜f",
            &[Consonant(Place(Labiodental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "b͜v",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜θ",
            &[Consonant(Place(Dental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "t̪͜θ",
            &[Consonant(Place(Dental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "d͜ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "d̪͜ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜s",
            &[Consonant(Place(Alveolar)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "d͜z",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜ʃ",
            &[Consonant(Place(PostAlveolar)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "d͜ʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "t͜ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
            ],
        ),
        (
            "d͜ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȶ͜ɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
            ],
        ),
        (
            "ȡ͜ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈ͜ʂ",
            &[Consonant(Place(Retroflex)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "ɖ͜ʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "c͜ç",
            &[Consonant(Place(Palatal)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "ɟ͜ʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "k͜x",
            &[Consonant(Place(Velar)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "g͜ɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "q͜χ",
            &[Consonant(Place(Uvular)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "ɢ͜ʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        // 2-char affricates
        (
            "pɸ",
            &[Consonant(Place(Bilabial)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "bβ",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "pf",
            &[Consonant(Place(Labiodental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "bv",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "tθ",
            &[Consonant(Place(Dental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "t̪θ",
            &[Consonant(Place(Dental)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "d̪ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ts",
            &[Consonant(Place(Alveolar)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "dz",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "tʃ",
            &[Consonant(Place(PostAlveolar)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "dʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "tɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
            ],
        ),
        (
            "dʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ȶɕ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
            ],
        ),
        (
            "ȡʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʈʂ",
            &[Consonant(Place(Retroflex)), Consonant(Manner(Affricate(Median, Sibilant)))],
        ),
        (
            "ɖʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Affricate(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "cç",
            &[Consonant(Place(Palatal)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "ɟʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "kx",
            &[Consonant(Place(Velar)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "gɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "qχ",
            &[Consonant(Place(Uvular)), Consonant(Manner(Affricate(Median, NonSibilant)))],
        ),
        (
            "ɢʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Affricate(Median, NonSibilant))),
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
            &[Consonant(Place(Bilabial)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "β",
            &[
                Consonant(Place(Bilabial)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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
            &[Consonant(Place(Labiodental)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "v",
            &[
                Consonant(Place(Labiodental)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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
        // dental
        (
            "θ",
            &[Consonant(Place(Dental)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "ð",
            &[
                Consonant(Place(Dental)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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
            &[Consonant(Place(Alveolar)), Consonant(Manner(Fricative(Median, Sibilant)))],
        ),
        (
            "z",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(Fricative(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ʃ",
            &[Consonant(Place(PostAlveolar)), Consonant(Manner(Fricative(Median, Sibilant)))],
        ),
        (
            "ʒ",
            &[
                Consonant(Place(PostAlveolar)),
                Consonant(Manner(Fricative(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɬ",
            &[Consonant(Place(Alveolar)), Consonant(Manner(LateralFricative))],
        ),
        (
            "ɮ",
            &[
                Consonant(Place(Alveolar)),
                Consonant(Manner(LateralFricative)),
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
                Consonant(Manner(Trill)),
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
                Consonant(Manner(Trill)),
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
                Consonant(Manner(Fricative(Median, Sibilant))),
            ],
        ),
        (
            "ʑ",
            &[
                Consonant(Place(Alveolopalatal)),
                Consonant(Manner(Fricative(Median, Sibilant))),
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
            &[Consonant(Place(Retroflex)), Consonant(Manner(Fricative(Median, Sibilant)))],
        ),
        (
            "ʐ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Fricative(Median, Sibilant))),
                Consonant(Voiced),
            ],
        ),
        (
            "ɽ",
            &[
                Consonant(Place(Retroflex)),
                Consonant(Manner(Trill)),
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
            &[Consonant(Place(Palatal)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "ʝ",
            &[
                Consonant(Place(Palatal)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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
            &[Consonant(Place(Velar)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "ɣ",
            &[
                Consonant(Place(Velar)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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
                Consonant(Voiced),
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
            &[Consonant(Place(Uvular)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "ʁ",
            &[
                Consonant(Place(Uvular)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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
        // pharyngeal
        (
            "ħ",
            &[Consonant(Place(Pharyngeal)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "ʕ",
            &[
                Consonant(Place(Pharyngeal)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
                Consonant(Voiced),
            ],
        ),
        // glottal
        ("ʔ", &[Consonant(Place(Glottal)), Consonant(Manner(Stop))]),
        (
            "h",
            &[Consonant(Place(Glottal)), Consonant(Manner(Fricative(Median, NonSibilant)))],
        ),
        (
            "ɦ",
            &[
                Consonant(Place(Glottal)),
                Consonant(Manner(Fricative(Median, NonSibilant))),
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

/// Modifiers which appear before the base character. This list is not guaranteed to be stable across versions.
pub static PREFIX_MODIFIERS: &[(&str, crate::Modifier)] = {
    use crate::Modifier::*;

    &[
        ("m͜", PreNasalized),
        ("n͜", PreNasalized),
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
        ("̪", Dental),
        ("͆", Dental),
        ("̼", Linguolabial),
        ("̺", Apical),
        ("̻", Laminal),
        ("̟", RelativeArticulation(crate::feature::RelativeArticulation::Advanced)),
        ("̠", RelativeArticulation(crate::feature::RelativeArticulation::Retracted)),
        ("˖", RelativeArticulation(crate::feature::RelativeArticulation::Advanced)),
        ("˗", RelativeArticulation(crate::feature::RelativeArticulation::Retracted)),
        ("̈", RelativeArticulation(crate::feature::RelativeArticulation::Centralized)),
        ("̽", RelativeArticulation(crate::feature::RelativeArticulation::MidCentralized)),
        ("̝", RelativeArticulation(crate::feature::RelativeArticulation::Raised)),
        ("˔", RelativeArticulation(crate::feature::RelativeArticulation::Raised)),
        ("̞", RelativeArticulation(crate::feature::RelativeArticulation::Lowered)),
        ("˕", RelativeArticulation(crate::feature::RelativeArticulation::Lowered)),
        ("̹", RelativeArticulation(crate::feature::RelativeArticulation::OverRounded)),
        ("͗", RelativeArticulation(crate::feature::RelativeArticulation::OverRounded)),
        ("̜", RelativeArticulation(crate::feature::RelativeArticulation::UnderRounded)),
        ("͑", RelativeArticulation(crate::feature::RelativeArticulation::UnderRounded)),
        ("ʷ", SecondaryArticulation(crate::feature::SecondaryArticulation::Labialized)),
        ("ʲ", SecondaryArticulation(crate::feature::SecondaryArticulation::Palatalized)),
        ("ˠ", SecondaryArticulation(crate::feature::SecondaryArticulation::Velarized)),
        ("̴", SecondaryArticulation(crate::feature::SecondaryArticulation::Velarized)),
        ("ˤ", SecondaryArticulation(crate::feature::SecondaryArticulation::Pharyngealized)),
        ("𐞴", SecondaryArticulation(crate::feature::SecondaryArticulation::Epiglottalized)),
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
    ]
};

/// See <https://en.wikipedia.org/wiki/Tone_letter>.
pub static TONE_LETTERS: &[(&str, crate::Modifier)] = {
    &[
        // stem to the right
        ("˥", crate::Modifier::Tone(crate::feature::Tone::ExtraHigh)),
        ("˦", crate::Modifier::Tone(crate::feature::Tone::High)),
        ("˧", crate::Modifier::Tone(crate::feature::Tone::Mid)),
        ("˨", crate::Modifier::Tone(crate::feature::Tone::Low)),
        ("˩", crate::Modifier::Tone(crate::feature::Tone::ExtraLow)),
        // stem to the left
        ("꜒", crate::Modifier::Tone(crate::feature::Tone::ExtraHigh)),
        ("꜓", crate::Modifier::Tone(crate::feature::Tone::High)),
        ("꜔", crate::Modifier::Tone(crate::feature::Tone::Mid)),
        ("꜕", crate::Modifier::Tone(crate::feature::Tone::Low)),
        ("꜖", crate::Modifier::Tone(crate::feature::Tone::ExtraLow)),
        // dotted tone letters
        ("꜈", crate::Modifier::Tone(crate::feature::Tone::ExtraHigh)),
        ("꜉", crate::Modifier::Tone(crate::feature::Tone::High)),
        ("꜊", crate::Modifier::Tone(crate::feature::Tone::Mid)),
        ("꜋", crate::Modifier::Tone(crate::feature::Tone::Low)),
        ("꜌", crate::Modifier::Tone(crate::feature::Tone::ExtraLow)),
        // dotted tone letters with stem to the left
        ("꜍", crate::Modifier::Tone(crate::feature::Tone::ExtraHigh)),
        ("꜎", crate::Modifier::Tone(crate::feature::Tone::High)),
        ("꜏", crate::Modifier::Tone(crate::feature::Tone::Mid)),
        ("꜐", crate::Modifier::Tone(crate::feature::Tone::Low)),
        ("꜑", crate::Modifier::Tone(crate::feature::Tone::ExtraLow)),
        // numerals
        ("5", crate::Modifier::Tone(crate::feature::Tone::ExtraHigh)),
        ("4", crate::Modifier::Tone(crate::feature::Tone::High)),
        ("3", crate::Modifier::Tone(crate::feature::Tone::Mid)),
        ("2", crate::Modifier::Tone(crate::feature::Tone::Low)),
        ("1", crate::Modifier::Tone(crate::feature::Tone::ExtraLow)),
        // superscript numerals
        ("⁵", crate::Modifier::Tone(crate::feature::Tone::ExtraHigh)),
        ("⁴", crate::Modifier::Tone(crate::feature::Tone::High)),
        ("³", crate::Modifier::Tone(crate::feature::Tone::Mid)),
        ("²", crate::Modifier::Tone(crate::feature::Tone::Low)),
        ("¹", crate::Modifier::Tone(crate::feature::Tone::ExtraLow)),
    ]
};