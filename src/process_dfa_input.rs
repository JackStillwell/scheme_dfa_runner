use std::collections::HashMap;

#[derive(Debug)]
struct DFA {
    start_state: String,
    transition_map: HashMap<Transition, String>,
    accept_states: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Transition {
    state: String,
    input: String,
}

#[derive(Debug, PartialEq)]
enum ReadSchemeDFA {
    Begin,
    ReadStart,
    OpenTransitions,
    OpenTransition1,
    OpenTransition2,
    ReadTransitionCurrent,
    ReadTransitionInput,
    ReadTransitionSep,
    ReadTransitionNext,
    ReadTransitionsSep,
    CloseTransitions,
    OpenAccepts,
    ReadAccept,
    Finish,
}

fn dfa_to_hashmap(dfa: String) -> Result<DFA, String> {
    let mut start_state = String::new();
    let mut transition_map = HashMap::new();
    let mut accept_states = Vec::new();

    let mut reader_state: ReadSchemeDFA = ReadSchemeDFA::Begin;
    let mut str_buffer: String = String::new();
    let mut current_state: String = String::new();
    let mut transition_input: String = String::new();
    let mut next_state: String = String::new();

    for c in dfa.chars() {
        match reader_state {
            ReadSchemeDFA::Begin => {
                if c != '(' {
                    break;
                }
                reader_state = ReadSchemeDFA::ReadStart;
            }
            ReadSchemeDFA::ReadStart => {
                if c == ' ' {
                    reader_state = ReadSchemeDFA::OpenTransitions;
                    start_state = str_buffer.clone();
                    str_buffer = String::new();
                    continue;
                }
                str_buffer.push(c);
            }
            ReadSchemeDFA::OpenTransitions => {
                if c != '(' {
                    break;
                }
                reader_state = ReadSchemeDFA::OpenTransition1;
            }
            ReadSchemeDFA::OpenTransition1 => {
                if c != '(' {
                    break;
                }
                reader_state = ReadSchemeDFA::OpenTransition2;
            }
            ReadSchemeDFA::OpenTransition2 => {
                if c != '(' {
                    break;
                }
                reader_state = ReadSchemeDFA::ReadTransitionCurrent;
            }
            ReadSchemeDFA::ReadTransitionCurrent => {
                if c == ' ' {
                    reader_state = ReadSchemeDFA::ReadTransitionInput;
                    current_state = str_buffer.clone();
                    str_buffer = String::new();
                    continue;
                }
                str_buffer.push(c);
            }
            ReadSchemeDFA::ReadTransitionInput => {
                if c == ')' {
                    reader_state = ReadSchemeDFA::ReadTransitionSep;
                    transition_input = str_buffer.clone();
                    str_buffer = String::new();
                    continue;
                }
                str_buffer.push(c);
            }
            ReadSchemeDFA::ReadTransitionSep => {
                if c != ' ' {
                    break;
                }
                reader_state = ReadSchemeDFA::ReadTransitionNext;
            }
            ReadSchemeDFA::ReadTransitionNext => {
                if c == ')' {
                    reader_state = ReadSchemeDFA::ReadTransitionsSep;
                    next_state = str_buffer.clone();
                    str_buffer = String::new();
                    continue;
                }
                str_buffer.push(c);
            }
            ReadSchemeDFA::ReadTransitionsSep => {
                if c == ')' {
                    reader_state = ReadSchemeDFA::CloseTransitions;
                    transition_map.insert(
                        Transition {
                            state: current_state.clone(),
                            input: transition_input.clone(),
                        },
                        next_state.clone(),
                    );
                    current_state = String::new();
                    transition_input = String::new();
                    next_state = String::new();
                } else if c == ' ' {
                    reader_state = ReadSchemeDFA::OpenTransition1;
                    transition_map.insert(
                        Transition {
                            state: current_state.clone(),
                            input: transition_input.clone(),
                        },
                        next_state.clone(),
                    );
                    current_state = String::new();
                    transition_input = String::new();
                    next_state = String::new();
                } else {
                    break;
                }
            }
            ReadSchemeDFA::CloseTransitions => {
                if c != ' ' {
                    break;
                }
                reader_state = ReadSchemeDFA::OpenAccepts;
            }
            ReadSchemeDFA::OpenAccepts => {
                if c != '(' {
                    break;
                }
                reader_state = ReadSchemeDFA::ReadAccept;
            }
            ReadSchemeDFA::ReadAccept => {
                if c == ' ' {
                    accept_states.push(str_buffer.clone());
                    str_buffer = String::new();
                    continue;
                } else if c == ')' {
                    accept_states.push(str_buffer.clone());
                    str_buffer = String::new();
                    reader_state = ReadSchemeDFA::Finish;
                    continue;
                }
                str_buffer.push(c);
            }
            ReadSchemeDFA::Finish => {
                if c != ')' {
                    return Err(
                        "Parsing Error in dfa_to_hashmap: incorrect final character".to_string()
                    );
                }
                continue;
            }
        }
    }

    if reader_state != ReadSchemeDFA::Finish {
        return Err(format!(
            "Parsing Error Occurred in dfa_to_hashmap with final state {:?}",
            reader_state
        ));
    }

    return Ok(DFA {
        start_state,
        transition_map,
        accept_states,
    });
}

pub fn process_schema(input: String, dfa: String) -> Result<bool, String> {
    let dfa = match dfa_to_hashmap(dfa) {
        Ok(d) => d,
        Err(error) => {
            return Err(error);
        }
    };

    /* println!("{:#?}", dfa); */

    let mut state: String = dfa.start_state;

    for c in input.chars() {
        let t = Transition {
            state: state.clone(),
            input: c.to_string(),
        };

        state = match dfa.transition_map.get(&t) {
            Some(s) => s.to_string(),
            None => continue,
        };
    }

    if dfa.accept_states.contains(&state) {
        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_schema_test() {
        assert_eq!(
            process_schema("b".to_string(), "(q0 (((q0 b) q1)) (q1))".to_string()),
            Ok(true)
        );
    }
}
