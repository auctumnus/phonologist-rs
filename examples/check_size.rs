fn main() {
    use phonologist::data::PHONEMES;

    let tuple_array = std::mem::size_of_val(PHONEMES);
    let mut string_bytes = 0usize;
    let mut feature_bytes = 0usize;

    for (s, features) in PHONEMES {
        string_bytes += s.len();
        feature_bytes += std::mem::size_of_val(*features);
    }

    let total = tuple_array + string_bytes + feature_bytes;
    println!("entries:        {:>8}", PHONEMES.len());
    println!("tuple array:    {:>8} bytes", tuple_array);
    println!("string data:    {:>8} bytes", string_bytes);
    println!("feature arrays: {:>8} bytes", feature_bytes);
    println!(
        "total logical:  {:>8} bytes  ({:.2} KB)",
        total,
        total as f64 / 1024.0
    );
}
