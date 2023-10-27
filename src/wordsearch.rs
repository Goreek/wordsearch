use build_html::*;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use widestring::ustring::WideString;
use widestring::WideChar;

pub struct WordSearch {
    width: usize,
    height: usize,
    board: Vec<Vec<WideChar>>,
    rng: StdRng,
    added: Vec<String>,
}

impl WordSearch {
    pub fn new(sz: usize, seed: u64) -> Self {
        WordSearch {
            width: sz,
            height: sz,
            board: vec![vec!['.' as WideChar; sz]; sz],
            rng: StdRng::seed_from_u64(seed),
            added: vec![],
        }
    }

    pub fn print_text(&self) {
        for line in self.get_board_strings().iter() {
            let str_line: String = line.iter().join(" ");
            println!("{}", str_line);
        }

        let footer = self.added.iter().join(" ");
        println!("");
        for w in textwrap::wrap(&footer, self.width * 2 - 1) {
            println!("{}", w);
        }
    }

    pub fn print_md(&self) {
        println!("Word Search:");
        println!("");
        println!("|{}", vec![" |"; self.width].join(""));
        let header = (0..self.width).into_iter().map(|_| "---").join(" | ");
        println!("| {} |", header);
        for line in self.get_board_strings().iter() {
            let str_line: String = line.iter().join(" | ");
            println!("| {} |", str_line);
        }

        let footer = self.added.iter().join(" ");
        println!("");
        for w in textwrap::wrap(&footer, self.width * 2 - 1) {
            println!("{}", w);
        }
    }

    pub fn print_html(&self) {
        let table = Table::from(self.get_board_strings()).with_attributes([("class", "wsboard")]);
        let words_str = self.added.iter().join(" ");
        let html = HtmlPage::new()
            .with_title("Word Search")
            .with_style(
                r#"div.wordsearch{width: min-content;} 
        table{table-layout: fixed; border-collapse: collapse;} 
        td{font-size: 30px; border: 1px solid black; text-align: center;} 
        p{width: fit-content;font-size: 20px;}"#,
            )
            .with_header(1, "Word Search:")
            .with_container(
                Container::default()
                    .with_attributes([("class", "wordsearch")])
                    .with_table(table)
                    .with_paragraph(words_str),
            );
        println!("{}", html.to_html_string());
    }

    fn get_board_strings(&self) -> Vec<Vec<String>> {
        self.board
            .iter()
            .map(|r| {
                r.iter()
                    .map(|&c| WideString::from_vec(vec![c]).to_string_lossy())
                    .collect()
            })
            .collect()
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
            _ => self.rng.gen_range(0..=1) * 2 - 1,
        };
        let x0 = match hd {
            1 => self.rng.gen_range(0..(self.width - word.len())),
            -1 => self.rng.gen_range((word.len() - 1)..self.width),
            _ => self.rng.gen_range(0..self.width),
        };

        if need_shared {
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

        true
    }

    pub fn add_word(&mut self, word: String) -> bool {
        let wstr = WideString::from_str(&word);
        if wstr.len() <= self.width && wstr.len() <= self.height {
            if !self.added.is_empty() {
                for _ in 0..self.width * self.height {
                    if self.try_add_word(&wstr, true) {
                        self.added.push(word);
                        return true;
                    }
                }
            }

            for _ in 0..self.width + self.height {
                if self.try_add_word(&wstr, false) {
                    self.added.push(word);
                    return true;
                }
            }
        }

        false
    }

    pub fn fill_random(&mut self, noize: String) {
        let wstr_noize = WideString::from_str(noize.as_str());
        let syms = wstr_noize.as_vec();
        for line in self.board.iter_mut() {
            for c in line.iter_mut() {
                if *c == '.' as WideChar {
                    *c = syms[self.rng.gen_range(0..syms.len())];
                }
            }
        }
    }
}
