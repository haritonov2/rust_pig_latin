pub mod pig_latin {
    pub fn translate(text: &str) -> String {
        let lower_text = text.to_lowercase();
        let words = lower_text.split_whitespace();
        let mut result = String::new();

        for word in words {
            if word.len() == 0 {
                continue;
            }

            let letters: Vec<char> = word.chars().collect();

            if is_vowel(&letters[0]) {
                result = format!("{} {}", result, get_vowel_word(&letters));
            } else {
                result = format!("{} {}", result, get_consonant_word(&letters));
            }
        }

        result.trim().to_string()
    }

    fn is_vowel(c: &char) -> bool {
        ['a', 'e', 'i', 'o', 'u'].contains(c)
    }

    fn get_vowel_word(letters: &Vec<char>) -> String {
        let word: String = letters.iter().collect();

        word + "hay"
    }

    fn get_consonant_word(letters: &Vec<char>) -> String {
        let mut construct: Vec<char> = letters[1..letters.len()].iter().cloned().collect();

        construct.push(letters[0]);
        construct.push('a');
        construct.push('y');

        let word: String = construct.iter().collect();

        word
    }
}

#[cfg(test)]
mod tests {
    use super::pig_latin;

    #[test]
    fn test_couple_vowel_words() {
        assert_eq!(pig_latin::translate("egg and apple"), "egghay andhay applehay");
    }

    #[test]
    fn test_couple_consonant_words() {
        assert_eq!(pig_latin::translate("first egg user"), "irstfay egghay userhay");
    }

    #[test]
    fn test_sentence_long_words() {
        assert_eq!(pig_latin::translate("How are you doing"), "owhay arehay ouyay oingday");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(pig_latin::translate("multiple    spaces"), "ultiplemay pacessay");
    }

    #[test]
    fn test_one_letter_words() {
        assert_eq!(pig_latin::translate("I am a boy"), "ihay amhay ahay oybay");
    }
}
