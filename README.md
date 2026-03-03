# phonologist

A library for parsing IPA phonemes into their features and modifiers.
The main function is `Phoneme::parse`, which takes an IPA string and
returns a `Phoneme` struct containing the features and modifiers of
the phoneme, as well as any warnings that were generated during parsing.

Note that the parsing is a best-effort attempt, and may not be perfect.
Phonological notation is a method of communication, not a formal system,
and so care must be taken with the results of parsing.

## Usage

```rust
use phonologist::{Phoneme, feature::{Feature, ConsonantFeature, Manner}};

let (phoneme, warnings) = Phoneme::parse("t");
assert!(warnings.is_empty());
assert_eq!(phoneme.name(), "voiceless alveolar stop");
assert!(
    phoneme
        .features()
        .contains(&Feature::Consonant(ConsonantFeature::Manner(Manner::Stop)))
);
```

## License

MIT or Apache 2.0