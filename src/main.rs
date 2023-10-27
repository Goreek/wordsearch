use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

mod wordsearch;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum PrintLayout
{
    Text,
    Html,
    Markdown
}

impl std::fmt::Display for PrintLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Text => "text",
            Self::Html => "html",
            Self::Markdown => "md",
        };
        s.fmt(f)
    }
}
impl std::str::FromStr for PrintLayout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "html" => Ok(Self::Html),
            "md" => Ok(Self::Markdown),
            _ => Err(format!("Unknown print layout: {s}")),
        }
    }
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file with parameters for word search
    #[arg(short, long)]
    input: String,

    /// Output print as html
    #[arg(short, long, default_value_t = PrintLayout::Text)]
    layout: PrintLayout,

    /// Skip filling unused cells
    #[arg(long, default_value_t = false)]
    noizeless: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct WordSearchInput {
    size: usize,
    seed: u64,
    #[serde(default = "noize_en")]
    noize: String,
    words: Vec<String>,
}

fn noize_en() -> String {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()
}

impl Default for WordSearchInput {
    fn default() -> Self {
        WordSearchInput {
            size: 10,
            seed: 42,
            noize: noize_en(),
            words: ["EXAMEN", "MISTAKE", "TEACHER"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

fn get_default_input_filepath() -> std::io::Result<PathBuf> {
    match env::current_exe() {
        Ok(exe_path) => {
            let input_filepath = exe_path
                .parent()
                .unwrap()
                .join("../../inputs/word_search_1.yaml");

            Ok(input_filepath)
        }
        Err(e) => Err(e),
    }
}

fn read_input_file(filepath: PathBuf) -> Result<WordSearchInput, std::io::Error> {
    let contents =
        std::fs::read_to_string(filepath).expect("Should have been able to read the file");
    let input_file: WordSearchInput = serde_yaml::from_str::<WordSearchInput>(&contents).unwrap();

    Ok(input_file)
}

fn main() {
    let args: Args = Args::parse();
    //let args: Vec<String> = env::args().collect();

    let input_path = if args.input.is_empty() {
        get_default_input_filepath().unwrap()
    } else {
        PathBuf::from(args.input)
    };

    let input = read_input_file(input_path).expect("Failed to read input file");

    let mut ws = wordsearch::WordSearch::new(input.size, input.seed);
    input.words.into_iter().for_each(|w| {
        ws.add_word(w);
    });

    if !args.noizeless {
        ws.fill_random(input.noize);
    }

    match args.layout {
        PrintLayout::Text => {ws.print_text();},
        PrintLayout::Html => {ws.print_html();}
        PrintLayout::Markdown => {ws.print_md();},
    }

}
