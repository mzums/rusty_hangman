use random_word::Lang;
use std::io::{self, Write};

fn main() {
    let word = random_word::gen(Lang::En);
    let word_chars: Vec<char> = word.chars().collect();
    let mut revealed = vec!['_'; word_chars.len()];
    let mut chances = 7;

    while chances > 0 && revealed.contains(&'_') {
        for &c in &revealed {
            print!("{} ", c);
        }
        println!();
        print!("Chose a letter: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(ch) = input.trim().chars().next() {
            if word_chars.contains(&ch) {
                println!("Correct!");
                for (i, &wc) in word_chars.iter().enumerate() {
                    if wc == ch {
                        revealed[i] = ch;
                    }
                }
            } else {
                chances -= 1;
                println!("Incorrect! Left chances: {chances}");
            }
        } else {
            println!("Incorrect letter!");
        }
        println!();
    }

    if !revealed.contains(&'_') {
        println!("Congrats! You guessed the word: {word}");
    } else {
        println!("You lost! The word was: {word}");
    }
}
