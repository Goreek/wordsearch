
use rand::{Rng, SeedableRng};
use rand::rngs::{ StdRng };


struct WordSearch{
    width: usize,
    height: usize,
    board: Vec<Vec<char>>,
    rng: StdRng,
    empty: bool
}

impl WordSearch {
    pub fn new( sz: usize ) -> Self {
        WordSearch{ width: sz, height: sz, board: vec![vec!['.'; sz]; sz], rng: StdRng::seed_from_u64(43u64), empty: true }
    }

    pub fn print(&self){
        for line in self.board.iter() {
            println!("{:?}", line );
        }
    }

    fn idx(u: usize, i: usize, d: i32) -> usize {
        match d {
            -1 => u - i,
            1 => u + i,
            _ => u
        }
    }

    fn try_add_word(&mut self, word: &String, need_shared: bool) -> bool {
        let vd:i32 = self.rng.gen_range(-1..=1);
        let y0 = match vd { 
            1 => self.rng.gen_range(0..(self.height - word.len())),
            -1 => self.rng.gen_range((word.len()-1)..self.height),
            _ => self.rng.gen_range(0..self.height)
        };
        let hd:i32 =  match vd {
            1|-1 => self.rng.gen_range(-1..=1),
            _ => if self.rng.gen_bool(0.5) {1} else {-1}
        };
        let x0 = match hd { 
            1 => self.rng.gen_range(0..(self.width - word.len())),
            -1 => self.rng.gen_range((word.len()-1)..self.width),
            _ => self.rng.gen_range(0..self.width)
        };

        //println!("vd{},y0{},hd{},x0{}",vd,y0,hd,x0);

        if need_shared && !self.empty {
            if !word.chars().into_iter().enumerate().any(|(i,c)| { let bc = self.board[Self::idx(y0,i,vd)][Self::idx(x0,i,hd)]; bc == c }) {
                return false;
            }
        };

        if !word.chars().into_iter().enumerate().all(|(i,c)| { let bc = self.board[Self::idx(y0,i,vd)][Self::idx(x0,i,hd)]; bc == '.' || bc == c }) {
            return false;
        };

        for (i,&c) in word.as_bytes().iter().enumerate() {
            self.board[Self::idx(y0,i,vd)][Self::idx(x0,i,hd)] = c as char;
        }

        self.empty = false;

        true
    }

    pub fn add_word(&mut self, word: String) -> bool {
        for _ in 0..self.width*self.height {
            if self.try_add_word(&word, true) { return true }
        }

        for _ in 0..self.width+self.height {
            if self.try_add_word(&word, false) { return true }
        }

        false
    }

    pub fn fill_random(&mut self) {
        for line in self.board.iter_mut() {
            for c in line.iter_mut(){ 
                if *c == '.' {
                    *c = self.rng.gen_range('A'..'Z');
                }
            }
        }
    }
}


fn main() {
    let mut ws = WordSearch::new( 10 );
    ws.add_word(String::from("MAGIC"));
    ws.add_word(String::from("SCHOOLBUS"));
    ws.add_word(String::from("TEACHER"));
    ws.add_word(String::from("BOARD"));
    ws.add_word(String::from("BACKPACK"));
    ws.add_word(String::from("GENIUS"));
    ws.fill_random();
    ws.print();
}
