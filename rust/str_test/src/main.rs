fn process_vowel(word: String) -> String{
    let mut word = word;
    word.push_str(stringify!("-hay"));
    word
}

fn process_consonant(word: String) -> String{
    let mut ret = word.clone();
    let mut consentant = ' ';
    let mut idx = 0;
    for c in word.chars() {
        let c = c.to_ascii_uppercase();
        consentant = match c {
            'A' => ' ',
            'E' => ' ',
            'I' => ' ',
            'O' => ' ',
            'U' => ' ',
            _   => ret.remove(idx),
        };        
        idx += 1;

        if consentant != ' ' {
            break;
        }
    }

    ret.push('-');
    ret.push(consentant);
    ret.push_str("ay");
    ret
}

fn main() {
    let text = "first";

    let mut pig_latin_text = String::new();
    for c in text.chars() {
        let c = c.to_ascii_uppercase();
        pig_latin_text = match c {
            'A' => process_vowel(String::from(text)),
            'E' => process_vowel(String::from(text)),
            'I' => process_vowel(String::from(text)),
            'O' => process_vowel(String::from(text)),
            'U' => process_vowel(String::from(text)),
            _   => process_consonant(String::from(text)),
        };

        break;        
    }

    println!("{} -> {}", text, pig_latin_text);
}
