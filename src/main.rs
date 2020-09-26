mod note_utils;

use note_utils::find_note_on_fret;

use std::io::{stdin, stdout, Write};
use rand::{Rng, thread_rng};

fn main() {
    println!("Welcome!! let's get to know your fret board!!");
    let mut fastest_record = u8::max_value();
    loop {
        let string_idx = thread_rng().gen_range(0, 6);
        let fret_idx = thread_rng().gen_range(0, 13);

        print!("Here it is. String {} fret {} What is your guess?: ", 6 - string_idx, fret_idx);
        stdout().flush().unwrap();

        let timer = std::time::Instant::now();

        let mut ans = String::new();
        stdin().read_line(&mut ans).unwrap();
        let trimmed_ans = ans.trim_end();

        if trimmed_ans == "q" { break; }

        let actual = find_note_on_fret(string_idx, fret_idx);
        println!("Your guess is {}", trimmed_ans);
        if trimmed_ans == actual {
            println!("Correct!!");
        } else {
            println!("It's incorrect!! The answer is {}", actual);
        }

        let elapsed_sec = timer.elapsed().as_secs();
        println!("You took {} secs", elapsed_sec);
        if (elapsed_sec as u8) <= fastest_record {
            fastest_record = elapsed_sec as u8;
        }
    }
    println!("Let's play again later");
    println!("Your fastest guess was {} secs", fastest_record);
}

