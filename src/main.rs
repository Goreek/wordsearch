

struct WordSearch{
    board: Vec<Vec<char>>
}

impl WordSearch {
    pub fn new( sz: usize ) -> Self {
        WordSearch{ board: vec![vec!['.'; sz]; sz]}
    }

    pub fn print(&self){
        for line in self.board.iter() {
            println!("{:?}", line );
        }
    }
}


fn main() {
    println!("Hello, world!");

    let ws = WordSearch::new( 5 );
    ws.print();
}
