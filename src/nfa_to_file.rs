use std::io::Write;
use std::process::Command;
use std::{fs::File, path::Path};

use crate::{stage_2::rename_states, Nfa};

pub fn write_nfa_to_file(m: &Nfa, filename: &str) {
    let mut m = m.clone();
    rename_states(&mut m);

    let num_states = m.accepting_state + 1;

    let mut resulting_file_str = String::new();

    resulting_file_str.push_str(&format!("{}\n", num_states));

    for (start_state, out_transitions) in m.transitions.iter() {
        for (transition_character, target_state) in out_transitions {
            if *transition_character == '\0' {
                resulting_file_str.push_str(&format!("{} {} \\0\n", start_state, target_state));
            } else {
                resulting_file_str.push_str(&format!(
                    "{} {} {}\n",
                    start_state, target_state, transition_character
                ));
            }
        }
    }

    let path = Path::new(filename);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path).unwrap();
    file.write_all(resulting_file_str.as_bytes()).unwrap();
}

pub fn write_nfa_to_pdf(m: &Nfa) {
    write_nfa_to_file(m, "output.txt");
    Command::new("python3")
        .arg("./visualization.py")
        .output()
        .expect("Failed to execute python command");
}
