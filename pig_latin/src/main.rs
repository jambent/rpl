use std::io;


fn main() {
    println!("Enter string for conversion");
    let mut input_str = String::new();
    io::stdin()
    .read_line(&mut input_str)
    .expect("String could not be processed");

    let mut words:Vec<&str> = Vec::new();
    for word in input_str.split_whitespace() {
        words.push(word);
    }

    let mut pig_latin_words:Vec<String> = Vec::new();
    let vowels = ['a','e','i','o','u'];
    for word in words {
        let first_letter = match word.chars().next(){
                    Some(c) => c,
                    None => continue
                };
        if vowels.contains(&first_letter) {
            pig_latin_words.push(format!("{word}hay"));
        }
        else {
            let remainder: String = word.chars().skip(1).collect();
                pig_latin_words.push(format!("{}{}ay",remainder,first_letter));
        }
    }
    println!("{}",pig_latin_words.join(" "));
}
