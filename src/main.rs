/*
Copyright 2024 Tim Boudreau

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the “Software”),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE.
*/
use const_format::formatcp;
use convert_case::Casing as _;
use std::{collections::BTreeMap, process::exit};

const CASE_SWITCH_SHORT: &str = "-c";
const CASE_SWITCH_LONG: &str = "--case";

const HELP_SWITCH_SHORT: &str = "-h";
const HELP_SWITCH_LONG: &str = "--help";

const EXAMPLES_SWITCH_SHORT: &str = "-x";
const EXAMPLES_SWITCH_LONG: &str = "--examples";

const OMIT_NEWLINE_SWITCH_SHORT: &str = "-o";
const OMIT_NEWLINE_SWITCH_LONG: &str = "--omit-newline";

const TITLE_CASE: &str = "title";
const UPPER_CASE: &str = "upper";
const LOWER_CASE: &str = "lower";
const TOGGLE_CASE: &str = "toggle";
const SNAKE_CASE: &str = "snake";
const UPPER_SNAKE_CASE: &str = "upper-snake";
const PASCAL_CASE: &str = "pascal";
const CAMEL_CASE: &str = "camel";
const UPPER_CAMEL_CASE: &str = "upper-camel";
const SCREAMING_SNAKE_CASE: &str = "screaming-snake";
const KEBAB_CASE: &str = "kebab";
const COBOL_CASE: &str = "cobol";
const UPPER_KEBAB_CASE: &str = "upper-kebab";
const TRAIN_CASE: &str = "train";
const FLAT_CASE: &str = "flat";
const UPPER_FLAT_CASE: &str = "upper-flat";
const ALTERNATING_CASE: &str = "alternating";

const ALL_CASE_NAMES: [&str; 17] = [
    TITLE_CASE,
    UPPER_CASE,
    LOWER_CASE,
    TOGGLE_CASE,
    SNAKE_CASE,
    UPPER_SNAKE_CASE,
    PASCAL_CASE,
    CAMEL_CASE,
    UPPER_CAMEL_CASE,
    SCREAMING_SNAKE_CASE,
    KEBAB_CASE,
    COBOL_CASE,
    UPPER_KEBAB_CASE,
    TRAIN_CASE,
    FLAT_CASE,
    UPPER_FLAT_CASE,
    ALTERNATING_CASE,
];

/// List of all case names used in help and error messages:
const ALL: &str = formatcp!(
    r#"{TITLE_CASE}, {UPPER_CASE}, {LOWER_CASE}, {TOGGLE_CASE}, {SNAKE_CASE}, {UPPER_SNAKE_CASE}, {PASCAL_CASE}, {CAMEL_CASE}, {UPPER_CAMEL_CASE}, {SCREAMING_SNAKE_CASE}, {KEBAB_CASE}, {COBOL_CASE}, {UPPER_KEBAB_CASE}, {TRAIN_CASE}, {FLAT_CASE}, {UPPER_FLAT_CASE}, {ALTERNATING_CASE}"#
);

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // trim the full command line
    let mut flags = Flags::default();
    match find_case_in_args(args.as_slice(), &mut flags) {
        Ok(case) => {
            let input = Input::new(args);
            if flags.show_examples {
                all_examples(input, flags);
                return;
            }
            let mut ct = 0_usize;
            input.each_string(|curr| {
                let output = curr.to_case(case);
                if ct > 0 {
                    print!(" {output}");
                } else {
                    print!("{output}");
                }
                ct += 1;
            });
            if !flags.omit_trailing_newline {
                println!();
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn print_help_and_exit() -> ! {
    let help = include_str!("help.txt");
    eprintln!("{}", help);
    exit(2)
}

const HORIZONTAL_BAR: char = '─';
const VERTICAL_INTERSECTION_LEFT: char = '├';
const VERTICAL_INTERSECTION_RIGHT: char = '┤';
const INTERSECTION: char = '┼';
const HORIZ_INTERSECTION_BOTTOM: char = '┴';
const HORIZ_INTERSECTION_TOP: char = '┬';
const TOP_LEFT_CORNER: char = '┌';
const TOP_RIGHT_CORNER: char = '┐';
const BOTTOM_LEFT_CORNER: char = '└';
const BOTTOM_RIGHT_CORNER: char = '┘';

fn all_examples(input: Input, _: Flags) {
    let mut fake_flags = Flags::default();
    let input_strings: Vec<String> = input.into();
    let mut mapped: BTreeMap<&'static str, String> = BTreeMap::new();
    let mut max_len = 0_usize;
    let mut max_case_len = 0_usize;
    for case in ALL_CASE_NAMES {
        max_case_len = max_case_len.max(case.chars().count());
        // Construct fake command line arguments, rather than duplicate the code.
        let fake_args = ["-c".to_string(), case.to_string()];
        let case_enum = find_case_in_args(fake_args.as_slice(), &mut fake_flags)
            .expect("Built in type not accepted");
        let mut out = String::new();
        for s in input_strings.iter() {
            if !out.is_empty() {
                out.push(' ');
            }
            let cnv = s.to_case(case_enum);
            max_len = max_len.max(cnv.chars().count());
            out.push_str(cnv.as_str());
        }
        mapped.insert(case, out);
    }
    let divider_position = 2 + max_case_len + 1;
    let min_width = divider_position + 2 + max_len + 2;
    let mut divider = String::new();
    let mut divider_top = String::new();
    let mut divider_bottom = String::new();
    divider_top.push(TOP_LEFT_CORNER);
    divider_bottom.push(BOTTOM_LEFT_CORNER);
    divider.push(VERTICAL_INTERSECTION_LEFT);
    for i in 0..min_width - 2 {
        if i == divider_position {
            divider.push(INTERSECTION);
            divider_top.push(HORIZ_INTERSECTION_TOP);
            divider_bottom.push(HORIZ_INTERSECTION_BOTTOM);
        } else {
            divider.push(HORIZONTAL_BAR);
            divider_top.push(HORIZONTAL_BAR);
            divider_bottom.push(HORIZONTAL_BAR);
        }
    }
    divider.push(VERTICAL_INTERSECTION_RIGHT);
    divider_top.push(TOP_RIGHT_CORNER);
    divider_bottom.push(BOTTOM_RIGHT_CORNER);
    divider.push('\n');
    divider_top.push('\n');
    divider_bottom.push('\n');
    let divider_char_count = divider.chars().count();

    print!("{}", divider_top);
    let mut ct = 0_usize;
    let max = mapped.len();
    for (case, val) in mapped {
        let mut text = format!("│ {} ", case);
        while text.chars().count() <= divider_position {
            text.push(' ');
        }
        text.push_str("│ ");
        text.push_str(val.as_str());
        while text.chars().count() < divider_char_count - 2 {
            text.push(' ');
        }
        text.push_str("│\n");
        print!("{}", text);
        ct += 1;
        if ct == max {
            print!("{}", divider_bottom);
        } else {
            print!("{}", divider);
        }
    }
}

fn find_case_in_args(
    args: &[String],
    flags: &mut Flags,
) -> Result<convert_case::Case, &'static str> {
    let mut expecting_case = false;
    let mut result: Option<convert_case::Case> = None;
    for vv in args {
        match vv.as_str() {
            CASE_SWITCH_SHORT | CASE_SWITCH_LONG => {
                if expecting_case {
                    // return Err("-c or --case passed more than once");
                    return Err(formatcp!(
                        "-{CASE_SWITCH_SHORT} or --{CASE_SWITCH_LONG} passed more than once"
                    ));
                }
                expecting_case = true;
            }
            HELP_SWITCH_SHORT | HELP_SWITCH_LONG => {
                print_help_and_exit();
            }
            other => {
                if flags.process(other) {
                    continue;
                }
                if result.is_some() {
                    // Ignore anything after unknown args which will be treated as
                    // input.
                    break;
                }
                if expecting_case {
                    match other {
                        TITLE_CASE => result = Some(convert_case::Case::Title),
                        UPPER_CASE => result = Some(convert_case::Case::Upper),
                        LOWER_CASE => result = Some(convert_case::Case::Lower),
                        TOGGLE_CASE => result = Some(convert_case::Case::Toggle),
                        SNAKE_CASE => result = Some(convert_case::Case::Snake),
                        UPPER_SNAKE_CASE => result = Some(convert_case::Case::UpperSnake),
                        PASCAL_CASE => result = Some(convert_case::Case::Pascal),
                        CAMEL_CASE => result = Some(convert_case::Case::Camel),
                        UPPER_CAMEL_CASE => result = Some(convert_case::Case::UpperCamel),
                        SCREAMING_SNAKE_CASE => result = Some(convert_case::Case::ScreamingSnake),
                        KEBAB_CASE => result = Some(convert_case::Case::Kebab),
                        COBOL_CASE => result = Some(convert_case::Case::Cobol),
                        UPPER_KEBAB_CASE => result = Some(convert_case::Case::UpperKebab),
                        TRAIN_CASE => result = Some(convert_case::Case::Train),
                        FLAT_CASE => result = Some(convert_case::Case::Flat),
                        UPPER_FLAT_CASE => result = Some(convert_case::Case::UpperFlat),
                        ALTERNATING_CASE => result = Some(convert_case::Case::Alternating),
                        _ => return Err(formatcp!("Unknown case name. Valid cases: {}", ALL)),
                    }
                }
            }
        }
    }
    if let Some(r) = result {
        Ok(r)
    } else {
        Ok(convert_case::Case::Title)
    }
}

/// Determine if there are trailing command-line arguments which are not recognized as
/// flags and should be treated as input, and return that if present.
fn find_vec_input(args: Vec<String>) -> Option<Vec<String>> {
    if args.is_empty() {
        return None;
    }
    let mut result = Vec::with_capacity(args.len());
    let mut flgs = Flags::default();
    let mut expecting_case = false;
    for s in args {
        match s.as_str() {
            CASE_SWITCH_SHORT | CASE_SWITCH_LONG => {
                if result.is_empty() {
                    expecting_case = true;
                    continue;
                }
            }
            other => {
                if expecting_case {
                    expecting_case = false;
                    continue;
                }
                if result.is_empty() && flgs.process(other) {
                    continue;
                }
            }
        }
        result.push(s);
    }
    if !result.is_empty() {
        return Some(result);
    }
    None
}

/// Input which can either be a concatenation of command-line arguments, or
/// stdin.
enum Input {
    Stdin,
    Cli(Vec<String>),
}

impl From<Input> for Vec<String> {
    fn from(value: Input) -> Self {
        match value {
            Input::Cli(v) => v,
            Input::Stdin => {
                let mut result = Vec::new();
                value.each_string(|s| result.push(s));
                result
            }
        }
    }
}

impl Input {
    fn new(args: Vec<String>) -> Self {
        if let Some(cli_in) = find_vec_input(args) {
            Self::Cli(cli_in)
        } else {
            Self::Stdin
        }
    }

    fn each_string(self, mut f: impl FnMut(String)) {
        match self {
            Self::Cli(v) => {
                for s in v {
                    f(s)
                }
            }
            Self::Stdin => {
                let inp = std::io::stdin();
                loop {
                    let mut s = String::new();
                    match inp.read_line(&mut s) {
                        Ok(ct) => {
                            if ct > 0 {
                                let ss = s.trim_end();
                                f(ss.to_string());
                            } else {
                                // input terminated
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            exit(3);
                        }
                    }
                }
            }
        }
    }
}

/// Handles -x and -o line switches
#[derive(Copy, Clone, Debug, Default)]
struct Flags {
    omit_trailing_newline: bool,
    show_examples: bool,
}

impl Flags {
    fn process(&mut self, st: &str) -> bool {
        match st {
            EXAMPLES_SWITCH_SHORT | EXAMPLES_SWITCH_LONG => {
                self.show_examples = true;
                true
            }
            OMIT_NEWLINE_SWITCH_SHORT | OMIT_NEWLINE_SWITCH_LONG => {
                self.omit_trailing_newline = true;
                true
            }
            _ => false,
        }
    }
}
