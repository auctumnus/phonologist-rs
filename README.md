# phonologist

Allows you to parse phonemes in the International Phonetic Alphabet.

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