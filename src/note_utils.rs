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