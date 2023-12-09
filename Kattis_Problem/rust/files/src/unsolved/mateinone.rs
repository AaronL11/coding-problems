use std::{str,error,fmt};
use std::io;
use io::{stdin,Stdin,BufRead,stdout,Stdout,BufWriter,Write};
use std::fmt::{Display,Formatter};

type Int = usize;
#[allow(dead_code)]
const MOD: Int = 1_000_000_007;


#[derive(Debug,Default)]
struct StopCode;

impl error::Error for StopCode {}

impl Display for StopCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Code failed to execute")
    }
}

impl From<io::Error> for StopCode {
    fn from(_: io::Error) -> StopCode { StopCode }
}

struct Scanner {
    scanin: Stdin,
    buffer: Vec<u8>,
    iterator: str::SplitAsciiWhitespace<'static>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            scanin: stdin(),
            buffer: Vec::new(),
            iterator: "".split_ascii_whitespace()
        }
    }
    fn next<T: str::FromStr>(&mut self) -> Result<T, StopCode> {
        self.get_str()?.parse::<T>().ok().ok_or(StopCode)
    }
    fn get_str(&mut self) -> Result<&str,StopCode> {
        loop {
            if let Some(input) = self.iterator.next() {
                return Ok(input)
            }
            self.buffer.clear();
            self.scanin.lock().read_until(b'\n',&mut self.buffer)?;
            self.iterator = unsafe {
                let slice = str::from_utf8_unchecked(&self.buffer);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }
    fn take<T: str::FromStr>(&mut self, n: usize) -> Result<Vec<T>,StopCode> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let i = self.next::<T>()?;
            result.push(i);
        }
        Ok(result)
    }
    // Only works after const generics were stabilized!!
    //fn take<T, const N: usize>(&mut self) -> Result<[T;N],StopCode>
    //where
    //    T: str::FromStr + Default + Copy,
    //    [T;N]: Default {
    //    let mut result: [T;N] = Default::default();
    //    for i in 0..N {
    //        let n = self.next::<T>()?;
    //        result[i] = n;
    //    }
    //   Ok(result) 
    //}
    fn take_tuple<T, V>(&mut self) -> Result<(T,V),StopCode>
    where
        T: str::FromStr,
        V: str::FromStr {
        Ok((
            self.next::<T>()?,
            self.next::<V>()?
        ))
    }
    fn take_tuple3<T, V, U>(&mut self) -> Result<(T,V,U),StopCode>
    where
        T: str::FromStr,
        V: str::FromStr,
        U: str::FromStr {
        Ok((
            self.next::<T>()?,
            self.next::<V>()?,
            self.next::<U>()?
        ))
    }
}

#[derive(Debug,Copy,Clone)]
enum Team {
    EMPTY,
    White(Piece),
    Black(Piece)
}

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
enum Piece {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King
}

struct Board([[Team;8];8]);

impl Board {
    fn from_map<F: FnMut((usize,&mut [Team;8]))>(f: F) -> Self {
        let mut board = [[Team::EMPTY;8];8];
        board.iter_mut()
            .enumerate()
            .for_each(f);
        Self(board)
    }
    fn move(&self, piece: Piece, (x,y): (usize,usize)) {
        match piece {
            Piece::Queen => {
            }
            Piece::King => {
                for i in 0..8 {
                    for j in 0..8 {
                        if x>i && y>j {
                                match (x+i,y+i) {
                                    (a@1..=7,b@1..=7) => {
                                        if self.check(a,b) {
                                            break
                                        }
                                    }
                                    _ => continue
                                }
                                match (x-i,y-i) {
                                    (a@1..=7,b@1..=7) => {
                                        if self.check(a,b) {
                                            break
                                        }
                                    }
                                    _ => continue
                                }
                            }
                    }
                }
            }
            Piece::Rook => {}
            Piece::Knight => {} 
            Piece::Bishop => {}
            Piece::Pawn => {}
        }
        fn check(&self,i: usize, j:usize) -> bool {
            if self.0[i][j] 
        }
    }
}

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    use crate::{Team::*,Piece::*};
    use std::collections::HashMap;
    let mut pieces: HashMap<Piece,Vec<(usize,usize)>> = HashMap::with_capacity(6);
    let mut board = Board::from_map(
        |(i,row)|
                scan.get_str().unwrap()
                    .chars()
                    .zip(row.iter_mut().enumerate())
                    .for_each(|(chr,(j,col))| {
                            let team = match chr {
                                '.' => EMPTY,
                                'K' => White(King),
                                'Q' => White(Queen),
                                'R' => White(Rook),
                                'N' => White(Knight),
                                'B' => White(Bishop),
                                'P' => White(Pawn),
                                'k' => Black(King),
                                'q' => Black(Queen),
                                'r' => Black(Knight),
                                'n' => Black(Knight),
                                'b' => Black(Bishop),
                                'p' => Black(Pawn),
                                _ => unreachable!()
                            };
                            match team {
                                White(piece) => {
                                    if let Some(v) = pieces.get_mut(&piece) {
                                        v.push((i,j));
                                    } else {
                                        pieces.insert(piece,vec![(i,j)]);
                                    }
                                }
                                _ => ()
                            }
                            *col = team;
                        }
                        )
        );
    board.0.iter().for_each(|row|println!("{:?}",row));
    println!("{:?}",pieces);
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

