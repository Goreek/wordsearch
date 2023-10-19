use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use wchar::*;
use widestring::ustring::WideString;
use widestring::WideChar;

struct WordSearch {
    width: usize,
    height: usize,
    board: Vec<Vec<wchar_t>>,
    rng: StdRng,
    empty: bool,
}

impl WordSearch {
    pub fn new(sz: usize, seed: u64) -> Self {
        WordSearch {
            width: sz,
            height: sz,
            board: vec![vec!['.' as WideChar; sz]; sz],
            rng: StdRng::seed_from_u64(seed),
            empty: true,
        }
    }

    pub fn print(&self) {
        for line in self.board.iter() {
            let str_line: Vec<String> = line
                .iter()
                .map(|&c| WideString::from_vec(vec![c]).to_string_lossy())
                .collect();
            println!("{:?}", str_line);
        }
    }

    fn idx(u: usize, i: usize, d: i32) -> usize {
        match d {
            -1 => u - i,
            1 => u + i,
            _ => u,
        }
    }

    fn try_add_word(&mut self, word: &WideString, need_shared: bool) -> bool {
        let vd: i32 = self.rng.gen_range(-1..=1);
        let y0 = match vd {
            1 => self.rng.gen_range(0..(self.height - word.len())),
            -1 => self.rng.gen_range((word.len() - 1)..self.height),
            _ => self.rng.gen_range(0..self.height),
        };
        let hd: i32 = match vd {
            1 | -1 => self.rng.gen_range(-1..=1),
            _ => {
                if self.rng.gen_bool(0.5) {
                    1
                } else {
                    -1
                }
            }
        };
        let x0 = match hd {
            1 => self.rng.gen_range(0..(self.width - word.len())),
            -1 => self.rng.gen_range((word.len() - 1)..self.width),
            _ => self.rng.gen_range(0..self.width),
        };

        if need_shared && !self.empty {
            if !word.chars_lossy().into_iter().enumerate().any(|(i, c)| {
                let bc = self.board[Self::idx(y0, i, vd)][Self::idx(x0, i, hd)];
                bc == c as WideChar
            }) {
                return false;
            }
        };

        if !word.chars_lossy().into_iter().enumerate().all(|(i, c)| {
            let bc = self.board[Self::idx(y0, i, vd)][Self::idx(x0, i, hd)];
            bc == '.' as WideChar || bc == c as WideChar
        }) {
            return false;
        };

        for (i, c) in word.chars_lossy().into_iter().enumerate() {
            self.board[Self::idx(y0, i, vd)][Self::idx(x0, i, hd)] = c as WideChar;
        }

        self.empty = false;

        true
    }

    pub fn add_word(&mut self, word: WideString) {
        if word.len() <= self.width && word.len() <= self.height {
            for _ in 0..self.width * self.height {
                if self.try_add_word(&word, true) {
                    return;
                }
            }

            for _ in 0..self.width + self.height {
                if self.try_add_word(&word, false) {
                    return;
                }
            }
        }

        panic!("Can't fit word {:#} into board", word.to_string().unwrap());
    }

    pub fn fill_random(&mut self) {
        for line in self.board.iter_mut() {
            for c in line.iter_mut() {
                if *c == wch!('.') {
                    *c = self.rng.gen_range(wch!('A')..wch!('Z'));
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WordSearchInput {
    size: usize,
    seed: u64,
    words: Vec<String>,
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

    let input = read_input_file(input_path).unwrap();

    let mut ws = WordSearch::new(input.size, input.seed);
    for word in input.words.iter() {
        ws.add_word(WideString::from(word.as_str()));
    }
    ws.fill_random();
    ws.print();
}
