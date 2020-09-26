mod note_utils;

use note_utils::find_note_on_fret;

use std::io::{stdin, stdout, Write};
use rand::{Rng, thread_rng};

fn check_result(ans: &str, actual: &str) {
    println!("Your guess is {}", ans);
    if ans == actual {
        println!("Correct!!");
    } else {
        println!("It's incorrect!! The answer is {}", actual);
    }
}

fn elapsed_time(func: fn()) -> u8 {
    let timer = std::time::Instant::now();
    func();
    let elapsed_sec = timer.elapsed().as_secs();
    println!("You took {} secs", elapsed_sec);
    elapsed_sec as u8
}

fn main() {
    println!("Welcome!! let's get to know your fret board!!");
    let mut fastest_record = u8::max_value();
    loop {
        let elapsed_sec = elapsed_time(|| {
            let string_idx = thread_rng().gen_range(0, 6);
            let fret_idx = thread_rng().gen_range(0, 13);

            print!("Here it is. String {} fret {} What is your guess?: ", 6 - string_idx, fret_idx);
            stdout().flush().unwrap();
            let mut res = String::new();
            stdin().read_line(&mut res).unwrap();
            let inp = res.trim_end().clone();

            let actual = find_note_on_fret(string_idx, fret_idx);
            check_result(inp, actual);
        });
        
        if elapsed_sec <= fastest_record {
            fastest_record = elapsed_sec;
        }
    }
}

