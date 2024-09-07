# Rust Pig Latin Converter

This is a simple Rust library for converting strings to Pig Latin. The first consonant of each word is moved to the end of the word with "ay" added (e.g., "car" becomes "ar-cay"). Words that start with a vowel have "hay" added to the end (e.g., "apple" becomes "apple-hay").

## Usage

To use this library in your Rust project, add the following to your `Cargo.toml` dependencies section:

```toml
[dependencies]
rust_pig_latin = { git = "https://github.com/your_username/rust_pig_latin" }
```

Then, in your Rust code:

```rust
use rust_pig_latin::pig_latin;

fn main() {
    let origin = "it is my new car";
    let pig_latin = pig_latin::translate(origin);

    println!("Origin: {}", origin);
    println!("Pig Latin: {}", pig_latin);
}
```

## Limitations
* This library currently does not support punctuation marks such as dots, commas, and other symbols.
* Make sure to handle special cases of punctuation outside the library.

## Running Examples
You can find example code in the examples/ folder. To run an example, use:

```bash
cargo run --example basic_usage
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.



