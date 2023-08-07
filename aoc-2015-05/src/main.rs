use std::fs;

fn text_contains_number_of_vowels(text: &str, number_of_vowels: i8) -> Result<bool, String> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if number_of_vowels > vowels.len().try_into().unwrap() {
        return Err("Cannot put a higher number of vowels than there are vowels".to_string());
    }

    let mut vowel_count = 0;

    for char in text.chars() {
        if vowels.contains(&char) {
            vowel_count = vowel_count + 1
        }
    }

    return Ok(vowel_count >= number_of_vowels);
}

fn text_has_two_consecutive_characters(text: &str) -> bool {
    let text_length = text.len();

    for n in 0..text_length - 1 {
        if text.chars().nth(n) == text.chars().nth(n + 1) {
            return true;
        }
    }

    return false;
}

fn text_contains_str_in_array(text: &str, arr: &[&str]) -> bool {
    arr.iter().any(|str| text.contains(str))
}

fn main() {
    let forbidden_strings = ["ab", "cd", "pq", "xy"];
    let filename = "data.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let lines = contents.lines();

    let mut nice_string_count = 0;
    for line in lines {
        let str_contains_three_vowels = text_contains_number_of_vowels(line, 3).unwrap();

        let str_contains_two_consecutive_characters = text_has_two_consecutive_characters(line);
        let str_contains_forbidden_characters =
            text_contains_str_in_array(line, &forbidden_strings);

        if str_contains_three_vowels
            && str_contains_two_consecutive_characters
            && !str_contains_forbidden_characters
        {
            nice_string_count = nice_string_count + 1;
        }
    }

    println!("{:?}", nice_string_count);
}
