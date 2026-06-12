use crate::keyboard::kb_constants::keys::{KbPosition, KeyAction};
use crate::keyboard::kb_constants::tokens::MASTER_DICTIONARY;
use crate::keyboard::layout::{Keyboard, Layer};
// use regex::Regex;

enum LineType {
    LayerLine,
    RemapLine,
    MacroLine,
    SkipLine,
}

fn match_line_type(line_str: Option<&str>) -> LineType {
    match line_str.chars().next() {
        Some('<') => LineType::LayerLine,
        Some('[') => LineType::RemapLine,
        Some('{') => LineType::MacroLine,
        _ => LineType::SkipLine,
    }
}

fn iterate_brackets(bracketed_str: &str) -> Vec<&'static str> {
    match match_line_type(bracketed_str) {
        LineType::LayerLine => {}
    }

    a = "hi"
}

pub fn parse_layout_file(raw_file: &str) {
    let mut active_layer = Layer::Base;
    // let simple_overwrite_re = Regex::new(r"\[([a-z0-9\.\+\-\=\*\/]+)\]{2}").unwrap();
    // let macro_re = Regex::new(r"");
    for line in raw_file.lines() {
        let trimmed_line = line.trim();

        // let first_char = trimmed_line.chars().next();
        match trimmed_line.chars().next() {
            None | Some('*') => continue,

            Some('<') => {
                active_layer =
                    Layer::from_string(&trimmed_line[1..trimmed_line.len() - 1]).unwrap();
                continue;
            }

            // TODO: Implement tap and hold case handling
            Some('[') => {
                if let Some((trigger, action)) = trimmed_line.split_once('>') {
                    // if let Some(re_captures) = simple_overwrite_re.captures(trimmed_line) {
                    // let overwrite_kb_position = KbPosition::get_position(&re_captures[0]);
                    // let new_action = KeyAction::Remap(&MASTER_DICTIONARY[&re_captures[1]]);
                    // let overwrite_kb_position = KbPosition::get_position(&re_captures[0]);
                    let remap_split = re_captures.split_once(">");
                    let remap_pos_str = &remap_split.split_once("[")[1].split_once("]")[0];
                    let remap_token_str = &remap_split.split_once("[")[1].split_once("]")[0];
                    let remap_position = KbPosition::get_position(remap_position);
                    let remap_token = KeyAction::Remap(&MASTER_DICTIONARY[remap_token_str]);

                    new_kb.set_override(active_layer, remap_position, remap_token);
                    // new_kb.set_override(active_layer, overwrite_kb_position.unwrap(), new_action);
                }
            }

            Some('{') => {
                // TODO: implement assignment
                // let
                // if let Some((trigger, actions)) = trimmed_line.split_once('>') {
                // let macro_split =
                // }
            }

            Some(unrecognized_char) => {
                println!(
                    "Warning: invalid syntax starting with '{}' on line {}",
                    unrecognized_char, trimmed_line
                );
            } // _ => {}
        }
    }
}
