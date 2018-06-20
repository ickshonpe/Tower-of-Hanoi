use std::str::FromStr;

struct Hanoi {
    number_of_discs: u32,
    data: [Vec<u32>; 3]
}

impl Hanoi {
    fn new(number_of_discs: u32) -> Self {
        Hanoi {
            number_of_discs,
            data: [(0 .. number_of_discs).rev().collect(), Vec::new(), Vec::new()]
        }
    }

    fn move_disc(&mut self, source: usize, target: usize) {
        let disc = self.data[source].pop().unwrap();
        self.data[target].push(disc);
    }

    fn legal_move(&mut self, a: usize, b: usize) {
        match self.top(a) {
            None => {
                match self.top(b){
                    None => return,
                    Some(_) => self.move_disc(b, a),
                }
            },
            Some(n) => {
                match self.top(b) {
                    None => {
                        self.move_disc(a, b)
                    },
                    Some(m) => {
                        if n < m {
                            self.move_disc(a, b)
                        } else {
                            self.move_disc(b, a)
                        }
                    },
                }
            },
        }
    }

    fn is_solved(&self) -> bool {
        self.data[2] == (0 .. self.number_of_discs).rev().collect::<Vec<u32>>()
    }

    fn top(&self, peg: usize) -> Option<u32> {
        match self.data[peg].last() {
            None => None,
            Some(&n) => Some(n)
        }
    }
}

impl std::fmt::Display for Hanoi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}\n{:?}\n{:?}\n", self.data[0], self.data[1], self.data[2])
    }
}

fn main() {
    let number_of_discs = u32::from_str(&std::env::args().skip(1).next().unwrap()).unwrap();
    let mut hanoi = Hanoi::new(number_of_discs);
    println!("{}", hanoi);
    let even_strategy = [[0,1],[0,2],[1,2]];
    let odd_strategy = [[0,2],[0,1],[1, 2]];
    let strategy = match number_of_discs % 2 { 0 => even_strategy, _ => odd_strategy };
    let mut moves = 0;
    'outer: loop {
        for [a, b] in strategy.iter() {
            moves += 1;
            hanoi.legal_move(*a, *b);
            println!("{}", hanoi);
            if hanoi.is_solved() { break 'outer }
        }
    }
    println!("Solved! in {} moves!", moves);
}



