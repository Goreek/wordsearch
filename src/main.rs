use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

mod wordsearch;

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
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() > 1 {
        PathBuf::from(args[1].clone())
    } else {
        get_default_input_filepath().unwrap()
    };

    let input = read_input_file(input_path).expect("Failed to read input file");

    let mut ws = wordsearch::WordSearch::new(input.size, input.seed);
    input.words.into_iter().for_each(|w| {ws.add_word(w);});
    ws.fill_random(input.noize);
    ws.print();
}
