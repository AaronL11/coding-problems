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

struct Player{
    rank: u8,
    stars: u8,
    streak: u8
}

impl Player {
    fn new() -> Self {
        Self { rank:25, stars:0, streak:0 }
    }
    fn add_win(&mut self) {
        let rank = self.rank;
        match rank {
            0 => {}
            1..=10 => self.update_rank_win(5),
            11..=15 => self.update_rank_win(4),
            16..=20 => self.update_rank_win(3),
            21..=25 => self.update_rank_win(2),
            _ => unimplemented!()
        }
    }
    fn add_loss(&mut self) {
        let (rank,stars) = (self.rank,self.stars);
        match rank {
            0 => {}
            1..=10 => self.update_rank_loss(5),
            11..=15 => self.update_rank_loss(4),
            16..=20 => {
                self.streak = 0;
                if stars == 0 {
                    if rank < 20 {
                        self.rank += 1;
                        self.stars = 2;
                    }
                } else {
                    self.stars -= 1;
                }
            }
            21..=25 => self.streak = 0,
            _ => unimplemented!()
        }
    }
    fn update_rank_win(&mut self, n: u8) {
        if self.rank >= 6 {
            self.streak += 1;
        } else {
            self.streak = 0;
        }
        self.stars += if self.streak >= 3 { 2 } else { 1 };
        if self.stars > n {
            self.rank -= 1;
            self.stars -= n;
        }
    }
    fn update_rank_loss(&mut self, s: u8) {
        self.streak = 0;
        if self.stars == 0 {
            self.rank += 1;
            self.stars = s-1;
        } else {
            self.stars -= 1;
        }
    }
}

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    let mut player = Player::new();
    for c in scan.get_str()?.bytes() {
        match c {
            b'W' => player.add_win(),
            b'L' => player.add_loss(),
            _ => unimplemented!()
        }
        if player.rank == 0 { break }
    }
    dbg!((player.rank,player.stars));
    let rank = player.rank;
    if rank == 0 {
        writeln!(out,"Legend")?;
    } else {
        writeln!(out,"{}",rank)?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

