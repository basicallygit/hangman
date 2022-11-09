use std::io::{stdout, Write, stdin};
use std::{thread, time};
use rand::Rng;

const STAGES: [&str; 7] = [
"  +---+
  |   |
      |
      |
      |
      |",
"  +---+
  |   |
  O   |
      |
      |
      |",
"  +---+
  |   |
  O   |
  |   |
      |
      |",
"  +---+
  |   |
  O   |
 /|   |
      |
      |",
"  +---+
  |   |
  O   |
 /|\\  |
      |
      |",
"  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |",
"  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |"];

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let wordlist: Vec<&str> = vec!["computer", "castle", "keyboard", "mansion", "elephant"];
    let mut current_stage: usize = 0;
    let mut input = String::new();
    let mut used_chars = String::new();
    let mut correct_chars = String::new();
    let word = wordlist[rand::thread_rng().gen_range(0..wordlist.len())];
    
    loop {
        clear_screen();
        println!("{}\n", STAGES[current_stage]);
        for ch in word.chars() {
            if correct_chars.contains(ch) {
                print!("{} ", ch);
            }
            else {
                print!("_ ");
            }
        }
        print!("          used: {}", used_chars);
        stdout().flush().unwrap();
        println!("\n");
        if current_stage == 6 {
            println!("You lose! the word was: {}", word);
            thread::sleep(time::Duration::from_millis(1000));
            break;
        }
        print!("guess: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .unwrap();
        let trimmed = input.trim();
        if trimmed == word {
            println!("\nYou win!");
            break;
        }
        else {
            if word.contains(&trimmed) && !correct_chars.contains(&trimmed) {
                correct_chars.push_str(&trimmed);
            }
            else {
                if trimmed.len() == 1 {
                    used_chars.push_str(&trimmed);
                }
                current_stage += 1;
            }
        }
        input = String::new();
    }
}
