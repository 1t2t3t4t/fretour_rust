use std::io::{stdin, stdout, Write};

use rand::{Rng, thread_rng};

const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D",
    "D#", "E", "F", "F#", "G", "G#"
];

const FRETS: [&str; 6] = ["E", "A", "D", "G", "B", "E"];

fn find_note(curr_note: &str) -> u8 {
    for (idx, &note) in NOTES.iter().enumerate() {
        if note == curr_note {
            return idx as u8;
        }
    }
    0
}

fn find_note_on_fret<'a>(string_idx: u8, fret_idx: u8) -> &'a str {
    let start_note = FRETS[string_idx as usize];
    let note_pos = find_note(start_note);
    let jump_to_note= (note_pos + fret_idx) % NOTES.len() as u8;
    NOTES[jump_to_note as usize]
}

fn main() {
    println!("Welcome!! let's get to know your fret board!!");
    loop {
        let string_idx = thread_rng().gen_range(0, 6);
        let fret_idx = thread_rng().gen_range(0, 13);

        print!("Here it is. String {} fret {} What is your guess?: ", 6 - string_idx, fret_idx);
        stdout().flush().unwrap();

        let timer = std::time::Instant::now();

        let mut ans = String::new();
        stdin().read_line(&mut ans).unwrap();
        let trimmed_ans = ans.trim_end();

        if trimmed_ans == "q" {
            break;
        }

        let actual = find_note_on_fret(string_idx, fret_idx);
        println!("Your guess is {}", trimmed_ans);
        if trimmed_ans == actual {
            println!("Correct!!");
        } else {
            println!("It's incorrect!! The answer is {}", actual);
        }
        println!("You took {} secs", timer.elapsed().as_secs());
    }
    println!("Let's play again later");
}

