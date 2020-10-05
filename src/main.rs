mod note_utils;

use note_utils::{find_note_on_fret, draw_frets};

use std::io::{stdin, stdout, Write};
use rand::{Rng, thread_rng};

fn check_result(ans: &str, actual: &str) -> bool {
    if ans == actual {
        println!("Correct!!");
    } else {
        println!("It's incorrect!! The answer is {}", actual);
    }
    ans == actual
}

fn read_input() -> String {
    let mut res = String::new();
    stdin().read_line(&mut res).unwrap();
    res.trim_end().to_owned()
}

fn elapsed_time(mut func: impl FnMut()) -> u8 {
    let timer = std::time::Instant::now();
    func();
    let elapsed_sec = timer.elapsed().as_secs();
    println!("You took {} secs", elapsed_sec);
    elapsed_sec as u8
}

fn gen_range(min_string: u8, max_string: u8) -> (u8, u8) {
    let string_idx = thread_rng().gen_range(min_string - 1, max_string);
    let fret_idx = thread_rng().gen_range(0, 13);
    (string_idx as u8, fret_idx as u8)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let min_string = if let Some(str) = args.get(1) { 
        str.parse::<u8>().unwrap()
    } else {
        1
    };
    let max_string = if let Some(str) = args.get(2) { 
        str.parse::<u8>().unwrap()
    } else {
        6
    };
    println!("Welcome!! let's get to know your fret board!!");
    let mut fastest_record = u8::max_value();
    let mut streak = 0u8;
    loop {
        let mut should_break = false;
        let elapsed_sec = elapsed_time(|| {
            let (string_idx, fret_idx) = gen_range(min_string, max_string);

            print!("Here it is. String {} fret {} What is your guess?: ", 6 - string_idx, fret_idx);
            stdout().flush().unwrap();
            println!("");
            draw_frets(string_idx, fret_idx);
            
            let inp = &read_input()[..];
            let actual = find_note_on_fret(string_idx, fret_idx);
            should_break = !check_result(inp, actual);
        });
        
        if elapsed_sec <= fastest_record {
            fastest_record = elapsed_sec;
        }
        if should_break { break } else { streak += 1; }
    }

    println!("Your fastest record was {} secs", fastest_record);
    println!("Your streaks was {} consecutive corrects", streak);
}
