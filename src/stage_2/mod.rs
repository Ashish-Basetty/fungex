use std::collections::HashMap;
mod tests;

use crate::{Nfa, RegexExpr, State};

pub fn convert_regex_to_nfa(expression: &RegexExpr) -> Nfa {
    match expression {
        RegexExpr::SingleChar(c) => {
            let transitions = HashMap::from([(0, vec![(*c, 1)])]);
            Nfa {
                initial_state: 0,
                accepting_state: 1,
                transitions,
            }
        }
        RegexExpr::Star(e) => {
            let mut m = convert_regex_to_nfa(e);
            let num_states = rename_states(&mut m);

            let old_acc_state = m.accepting_state;
            let old_init_state = m.initial_state;
            let new_init_state = num_states;
            let new_acc_state = num_states + 1;

            m.initial_state = new_init_state;
            m.accepting_state = new_acc_state;
            m.transitions.insert(
                old_acc_state,
                vec![('\0', old_init_state), ('\0', new_acc_state)],
            );
            m.transitions.insert(
                new_init_state,
                vec![('\0', old_init_state), ('\0', new_acc_state)],
            );

            rename_states(&mut m);
            return m;
        }
        RegexExpr::Concat(e1, e2) => {
            let mut m1 = convert_regex_to_nfa(e1);
            let mut m2 = convert_regex_to_nfa(e2);
            rename_nfa_states(&mut m1, &mut m2);

            // we can get rid of (forget) the accepting state of m1, and then redirect all
            // transitions into m1's accepting state into m2's initial state
            let m1_acc_state = m1.accepting_state;
            let m2_init_state = m2.initial_state;

            let mut resulting_nfa = Nfa {
                initial_state: m1.initial_state,
                accepting_state: m2.accepting_state,
                transitions: HashMap::new(),
            };
            for (s, out_transitions) in m1.transitions {
                // the transitions in m1 might need to be renamed.
                resulting_nfa.transitions.insert(
                    s,
                    out_transitions
                        .iter()
                        .map(|(c, new_s)| {
                            (
                                *c,
                                if *new_s != m1_acc_state {
                                    *new_s
                                } else {
                                    m2_init_state
                                },
                            )
                        })
                        .collect(),
                );
            }
            for (s, out_transitions) in m2.transitions {
                resulting_nfa.transitions.insert(s, out_transitions);
            }

            rename_states(&mut resulting_nfa);
            return resulting_nfa;
        }
        RegexExpr::Or(e1, e2) => {
            let mut m1 = convert_regex_to_nfa(e1);
            let mut m2 = convert_regex_to_nfa(e2);
            rename_nfa_states(&mut m1, &mut m2);

            // we can get rid of (forget) the accepting state and initial state of m1, and then
            // rename the transitions in m1 to use the accepting and initial states of m2
            let m1_acc_state = m1.accepting_state;
            let m1_init_state = m1.initial_state;

            let mut resulting_nfa = Nfa {
                initial_state: m2.initial_state,
                accepting_state: m2.accepting_state,
                transitions: HashMap::new(),
            };

            for (s, out_transitions) in m1.transitions {
                // the transitions in m1 might need to be renamed.
                let s = if s != m1_init_state {
                    s
                } else {
                    m2.initial_state
                };
                resulting_nfa.transitions.insert(
                    s,
                    out_transitions
                        .iter()
                        .map(|(c, new_s)| {
                            (
                                *c,
                                if *new_s != m1_acc_state {
                                    *new_s
                                } else {
                                    m2.accepting_state
                                },
                            )
                        })
                        .collect(),
                );
            }
            for (s, out_transitions) in m2.transitions {
                if !resulting_nfa.transitions.contains_key(&s) {
                    resulting_nfa.transitions.insert(s, out_transitions);
                } else {
                    resulting_nfa
                        .transitions
                        .get_mut(&s)
                        .unwrap()
                        .extend(out_transitions);
                }
            }

            rename_states(&mut resulting_nfa);
            return resulting_nfa;
        }
    }
}

/// if m1 has n_1 states and m2 has n_2 states then the result of calling this function
/// is that m1 should have states named 0, 1, 2, ..., (n_1-1) and m2 should have states
/// named n_1, n_1+1, ..., n_1+n_2-1
///
/// This ensures that the two nfas have distinct state names.
fn rename_nfa_states(m1: &mut Nfa, m2: &mut Nfa) {
    let mut next_state: State = 0;
    let mut m1_rename_map: HashMap<State, State> = HashMap::new();
    let mut m2_rename_map: HashMap<State, State> = HashMap::new();

    // use the get_all_state_references function to construct the two rename_maps.
    for s in get_all_state_references(m1) {
        if !m1_rename_map.contains_key(&s) {
            m1_rename_map.insert(s, next_state);
            next_state += 1;
        }
    }
    for s in get_all_state_references(m2) {
        if !m2_rename_map.contains_key(&s) {
            m2_rename_map.insert(s, next_state);
            next_state += 1;
        }
    }

    *m1 = rename_nfa_with_map(m1, &m1_rename_map);
    *m2 = rename_nfa_with_map(m2, &m2_rename_map);
}

/// If m has n unique states, then this function renames the states 0, 1, ..., (n-1)
/// and also returns n
pub fn rename_states(m: &mut Nfa) -> usize {
    let mut next_state: State = 0;
    let mut rename_map: HashMap<State, State> = HashMap::new();
    for s in get_all_state_references(m) {
        if !rename_map.contains_key(&s) {
            rename_map.insert(s, next_state);
            next_state += 1;
        }
    }

    *m = rename_nfa_with_map(m, &rename_map);

    let num_total_states = rename_map.len();
    return num_total_states;
}

fn get_all_state_references(m: &Nfa) -> Vec<State> {
    let mut states = Vec::new();
    states.push(m.initial_state);
    for (q, out_transitions) in m.transitions.iter() {
        states.push(*q);
        for (transition_char, next_state) in out_transitions {
            if *next_state != m.accepting_state {
                states.push(*next_state);
            }
        }
    }
    states.push(m.accepting_state);
    states
}

fn rename_nfa_with_map(m: &Nfa, rename_map: &HashMap<State, State>) -> Nfa {
    let new_init_state = *rename_map.get(&m.initial_state).unwrap();
    let new_acc_state = *rename_map.get(&m.accepting_state).unwrap();
    let mut new_m = Nfa {
        initial_state: new_init_state,
        accepting_state: new_acc_state,
        transitions: HashMap::new(),
    };
    for (state, transitions) in m.transitions.iter() {
        let new_state = *rename_map.get(state).unwrap();
        let new_outgoing_transitions = transitions
            .iter()
            .map(|(c, s)| (*c, *rename_map.get(s).unwrap()))
            .collect::<Vec<(char, State)>>();
        new_m
            .transitions
            .insert(new_state, new_outgoing_transitions);
    }

    new_m
}
