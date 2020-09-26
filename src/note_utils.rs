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

fn draw_fret() {
    let mut fret = String::from("0");
    for _ in 0..12 {
        fret = fret.add("|   ");
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

pub fn draw_frets() {
    draw_fret_marks();
    for _ in 0..6 {
        draw_fret();
    }
    draw_fret_marks();
}