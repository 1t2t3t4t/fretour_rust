use std::ops::Add;

const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D",
    "D#", "E", "F", "F#", "G", "G#"
];

const FRETS: [&str; 6] = ["E", "A", "D", "G", "B", "E"];

fn find_note_pos(curr_note: &str) -> u8 {
    for (idx, &note) in NOTES.iter().enumerate() {
        if note == curr_note {
            return idx as u8;
        }
    }
    0
}

pub fn find_note_on_fret<'a>(string_idx: u8, fret_idx: u8) -> &'a str {
    let start_note = FRETS[string_idx as usize];
    let note_pos = find_note_pos(start_note);
    let jump_to_note= (note_pos + fret_idx) % NOTES.len() as u8;
    NOTES[jump_to_note as usize]
}

fn draw_fret(fret_mark_idx: Option<u8>) {
    let mut start_text = "0";
    if let Some(i) = fret_mark_idx {
        if i == 0 {
            start_text = "X";
        }
    }
    let mut fret = String::from(start_text);
    for i in 1..=12 {
        if let Some(idx) = fret_mark_idx{
            let text = if idx == i { "| X " } else { "|   " };
            fret = fret.add(text);
        } else {
            fret = fret.add("|   ");
        }
    };
    fret = fret.add("|");
    println!("{}", fret);
}

fn draw_fret_marks() {
    let mut mark = String::from(" ");
    for i in 1..=12 {
        if i % 2 != 0 && i != 11 {
            mark = mark.add("  * ");
        } else if i == 12 {
            mark = mark.add(" ***");
        } else {
            mark = mark.add("    ");
        }
    };
    println!("{}", mark);
}

pub fn draw_frets(string_idx: u8, fret_idx: u8) {
    let inverted_string_idx = 5 - string_idx;
    draw_fret_marks();
    for i in 0..6 {
        let fret_mark_idx = if i == inverted_string_idx { Some(fret_idx) } else { None };
        draw_fret(fret_mark_idx);
    }
    draw_fret_marks();
}