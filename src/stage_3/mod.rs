mod tests;

use std::collections::{HashSet, VecDeque};

use crate::{Nfa, State};

/// Runs a NFA on an input string.
/// returns true if the NFA accepts the input string, and false otherwise.
pub fn run_nfa(nfa: &Nfa, input_string: &str) -> bool {
    let mut curr_states: HashSet<State> = HashSet::new();
    curr_states.insert(nfa.initial_state);
    do_epsilon_closure(&mut curr_states, &nfa);

    for char in input_string.chars() {
        do_transition(&mut curr_states, &nfa, char);
        do_epsilon_closure(&mut curr_states, &nfa);
    }

    return curr_states.contains(&nfa.accepting_state);
}

// Updates `states` to be the epsilon closure of `states`.
fn do_epsilon_closure(states: &mut HashSet<State>, nfa: &Nfa) {
    let mut states_to_visit: VecDeque<State> = VecDeque::new();
    for s in states.iter() {
        states_to_visit.push_back(*s);
    }

    while !states_to_visit.is_empty() {
        let s = states_to_visit.pop_front().unwrap();

        let out_epsilon_transitions = nfa
            .transitions
            .get(&s)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|(c, _)| *c == '\0')
            .map(|(c, s)| *s)
            .collect::<Vec<State>>();

        for next_state in out_epsilon_transitions {
            if !states.contains(&next_state) {
                states.insert(next_state);
                states_to_visit.push_back(next_state);
            }
        }
    }
}

fn do_transition(states: &mut HashSet<State>, nfa: &Nfa, next_char: char) {
    let mut new_states: HashSet<State> = HashSet::new();

    for s in states.iter() {
        let reachable_states = nfa
            .transitions
            .get(&s)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|(c, _)| *c == next_char)
            .map(|(c, s)| *s)
            .collect::<Vec<State>>();
        new_states.extend(reachable_states);
    }
    *states = new_states;
}
