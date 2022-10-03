// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)

    let mut game = Game::new(secret_word_chars);
    let mut wrong_times = 0;
    println!("Welcome to CS110L Hangman!");
    loop {
        let left_num = game.cal_left_letter();
        if left_num == 0 {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            return
        }

        println!("The word so far is {}", game.get_current_status_str());
        println!(
            "You have guessed the following letters: {}",
            game.get_guess_log_str()
        );
        println!("You have {} guesses left", left_num);

        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        if game.guess(get_first_char_of_str(guess)) {
            continue;
        } else {
            println!("Sorry, that letter is not in the word");
            wrong_times+=1;
        }
        if wrong_times >= NUM_INCORRECT_GUESSES {
            println!("Sorry, you ran out of guesses!");
            return
        }
    }
}

struct Game {
    current_status: Vec<char>,
    secret_word: Vec<char>,
    guess_log: Vec<char>,
}

impl Game {
    fn new(secret_word_chars: Vec<char>) -> Game {
        let current_status = vec!['-'; secret_word_chars.len()];
        let guess_log = Vec::new();

        Game {
            current_status: current_status,
            secret_word: secret_word_chars,
            guess_log: guess_log,
        }
    }

    fn get_current_status_str(&self) -> String {
        from_vec_to_str(&self.current_status)
    }

    fn get_guess_log_str(&self) -> String {
        from_vec_to_str(&self.guess_log)
    }

    fn cal_left_letter(&self) -> i32 {
        let mut count = 0;
        for &i in &self.current_status {
            if i == '-' {
                count += 1;
            }
        }
        count
    }

    fn guess(&mut self, c: char) -> bool {
        for i in 0..self.secret_word.len() {
            if self.secret_word[i] == c && self.current_status[i] == '-' {
                self.current_status[i] = c;
                return true;
            }
        }
        false
    }
}

fn from_vec_to_str(v: &Vec<char>) -> String {
    let mut ret = String::new();
    for &i in v {
        ret.push(i);
    }
    ret
}

fn get_first_char_of_str(val :String) -> char {
    val.chars().nth(0).unwrap()
}
