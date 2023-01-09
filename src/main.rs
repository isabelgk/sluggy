use std::fmt::Display;

use clap::{Parser, ValueEnum};
use rand::Rng;
use slug::slugify;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Position {
    /// Prepend the base name
    Prepend,
    /// Append the base name
    Append,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Prepend => write!(f, "prepend"),
            Position::Append => write!(f, "append"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Case {
    /// All lowercase letters
    Lowercase,
    /// All uppercase letters
    Uppercase,
    /// Mixed uppercase and lowercase letters
    Mixed,
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Case::Lowercase => write!(f, "lowercase"),
            Case::Uppercase => write!(f, "uppercase"),
            Case::Mixed => write!(f, "mixed"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum NumberPlacement {
    /// No numbers
    None,
    /// Allow numbers anywhere
    Anywhere,
    /// Anywhere but first
    NotFirst,
}

impl Display for NumberPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberPlacement::None => write!(f, "none"),
            NumberPlacement::Anywhere => write!(f, "anywhere"),
            NumberPlacement::NotFirst => write!(f, "not-first"),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Base name
    name: Option<String>,
    /// Prepend or append the base name
    #[arg(short, long, default_value_t = Position::Append)]
    position: Position,
    /// Sets the length of the random string
    #[arg(short, default_value_t = 8)]
    length: usize,
    /// Sets the case of the result
    #[arg(short, long, default_value_t = Case::Lowercase)]
    case: Case,
    /// Where to allow numbers
    #[arg(short, long, default_value_t = NumberPlacement::NotFirst)]
    numbers: NumberPlacement,
}

fn rand_string(len: usize, case: Case, number_placement: NumberPlacement) -> String {
    // https://stackoverflow.com/a/74953997
    const LOWER_CHAR: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPER_CHAR: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &[u8] = b"0123456789";

    let mut rng = rand::thread_rng();
    let charset: Vec<u8> = match case {
        Case::Lowercase => match number_placement {
            NumberPlacement::None => LOWER_CHAR.into(),
            _ => [LOWER_CHAR, NUMBERS].concat(),
        },
        Case::Uppercase => match number_placement {
            NumberPlacement::None => UPPER_CHAR.into(),
            _ => [UPPER_CHAR, NUMBERS].concat(),
        },
        Case::Mixed => match number_placement {
            NumberPlacement::None => [LOWER_CHAR, UPPER_CHAR].concat(),
            _ => [LOWER_CHAR, UPPER_CHAR, NUMBERS].concat(),
        },
    };

    let mut first_char = |c: &Vec<u8>| c[rng.gen_range(0..c.len())] as char;
    let mut result: String = match number_placement {
        // Allow a number as the first char
        NumberPlacement::Anywhere => first_char(&charset).into(),
        // Don't allow a number
        _ => match case {
            Case::Lowercase => first_char(&LOWER_CHAR.into()).into(),
            Case::Uppercase => first_char(&UPPER_CHAR.into()).into(),
            Case::Mixed => first_char(&[LOWER_CHAR, UPPER_CHAR].concat()).into(),
        },
    };

    let end: String = std::iter::repeat_with(|| charset[rng.gen_range(0..charset.len())] as char)
        .take(len - 1)
        .collect();
    result.push_str(&end);
    result
}

fn main() {
    let cli = Cli::parse();
    let mut rand_string = rand_string(cli.length, cli.case, cli.numbers);

    let result = match cli.name {
        Some(name) => match cli.position {
            Position::Prepend => {
                let mut n = name.clone();
                n.push('-');
                n.push_str(&rand_string);
                n
            }
            Position::Append => {
                rand_string.push('-');
                rand_string.push_str(&name);
                rand_string
            }
        },
        None => rand_string,
    };

    println!("{}", slugify(result));
}
