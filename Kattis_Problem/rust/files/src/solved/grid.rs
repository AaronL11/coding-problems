#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    error, fmt,
    fmt::{Display, Formatter},
    io,
    io::{stdin, stdout, BufRead, BufWriter, Bytes, Read, Stdin, Stdout, Write},
    str,
    str::FromStr,
};

type Int = isize;
type Uint = usize;
const MOD: Uint = 1_000_000_007;

#[derive(Debug, Default)]
struct StopCode;

impl error::Error for StopCode {}

impl Display for StopCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Code failed to execute")
    }
}

impl From<io::Error> for StopCode {
    fn from(_: io::Error) -> StopCode {
        StopCode
    }
}

struct Scanner<R>
where
    R: Read,
{
    bytes: Bytes<R>,
    buffer: Vec<u8>,
    split: str::SplitAsciiWhitespace<'static>,
}

#[allow(dead_code, unused_assignments)]
impl<R> Scanner<R>
where
    R: Read,
{
    fn new(bytes: Bytes<R>) -> Self {
        Self {
            bytes,
            buffer: Vec::new(),
            split: "".split_ascii_whitespace(),
        }
    }
    fn next<T: FromStr>(&mut self) -> Result<T, StopCode> {
        self.get_str()?.parse::<T>().ok().ok_or(StopCode)
    }
    fn get_str(&mut self) -> Result<&str, StopCode> {
        loop {
            if let Some(input) = self.split.next() {
                return Ok(input);
            }
            self.buffer.clear();
            loop {
                match self.bytes.next().ok_or(StopCode)? {
                    Ok(b'\n') => {
                        self.split = unsafe {
                            let slice = str::from_utf8_unchecked(&self.buffer);
                            std::mem::transmute(slice.split_ascii_whitespace())
                        };
                        break;
                    }
                    Ok(byte) => self.buffer.push(byte),
                    Err(_) => return Err(StopCode),
                }
            }
        }
    }
    fn line(&mut self) -> Result<String, StopCode> {
        loop {
            match self.bytes.next().ok_or(StopCode)? {
                Ok(b'\n') => {
                    let result = unsafe { str::from_utf8_unchecked(&self.buffer).to_owned() };
                    self.buffer.clear();
                    return Ok(result);
                }
                Ok(byte) => self.buffer.push(byte),
                Err(_) => return Err(StopCode),
            }
        }
    }
    fn lines(&mut self) -> LineIter<R> {
        LineIter::new(self)
    }
    fn words<T: FromStr>(&mut self) -> WordsIter<T, R> {
        WordsIter::new(self)
    }
    fn take<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        self.words::<T>().take(n).collect::<Vec<T>>()
    }
    // Only works after const generics were stabilized!!
    fn take_tuple<T, V>(&mut self) -> Result<(T, V), StopCode>
    where
        T: FromStr,
        V: FromStr,
    {
        Ok((self.next::<T>()?, self.next::<V>()?))
    }
    fn take_tuple3<T, V, U>(&mut self) -> Result<(T, V, U), StopCode>
    where
        T: FromStr,
        V: FromStr,
        U: FromStr,
    {
        Ok((self.next::<T>()?, self.next::<V>()?, self.next::<U>()?))
    }
}

struct WordsIter<'a, T: FromStr, R: Read>(&'a mut Scanner<R>, std::marker::PhantomData<T>);

impl<'a, T: FromStr, R: Read> WordsIter<'a, T, R> {
    fn new(scan: &'a mut Scanner<R>) -> Self {
        Self(scan, std::marker::PhantomData)
    }
}

impl<'a, T, R> Iterator for WordsIter<'a, T, R>
where
    T: FromStr,
    R: Read,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next::<T>().ok()
    }
}

struct LineIter<'a, R: Read>(&'a mut Scanner<R>);

impl<'a, R: Read> LineIter<'a, R> {
    fn new(scan: &'a mut Scanner<R>) -> Self {
        Self(scan)
    }
}

impl<'a, R> Iterator for LineIter<'a, R>
where
    R: Read,
{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.line().ok()
    }
}

#[allow(non_snake_case)]
fn solve<R>(mut scan: Scanner<R>, mut out: BufWriter<Stdout>) -> Result<(), StopCode>
where
    R: Read,
{
    let mut queue = VecDeque::with_capacity(500);
    let mut grid = [[(0, false); 501]; 501];
    let (n, m) = scan.take_tuple::<Uint, Uint>()?;
    let go = |dd, x, b| {
        (
            if x >= dd { Some(x - dd) } else { None },
            if x + dd < b { Some(x + dd) } else { None },
        )
    };
    for i in 0..n {
        for (j, byte) in scan.get_str()?.bytes().enumerate() {
            grid[i][j] = ((byte - 48) as usize, false);
        }
    }
    let mut distance = 0;
    let mut min = 1_000_000_007;
    queue.push_back((0, 0, distance));
    while let Some((r, c, d)) = queue.pop_front() {
        distance = d;
        let (dd, visited) = grid[r][c];
        if visited {
            continue;
        } else {
            grid[r][c] = (dd, true);
        }
        if (r, c) == (n - 1, m - 1) {
            min = cmp::min(min, distance);
            continue;
        }
        let (l, rt) = go(dd, c, m);
        if let Some(x) = l {
            if !grid[r][x].1 {
                queue.push_back((r, x, distance + 1));
            }
        }
        if let Some(x) = rt {
            if !grid[r][x].1 {
                queue.push_back((r, x, distance + 1));
            }
        }
        let (u, d) = go(dd, r, n);
        if let Some(x) = u {
            if !grid[x][c].1 {
                queue.push_back((x, c, distance + 1));
            }
        }
        if let Some(x) = d {
            if !grid[x][c].1 {
                queue.push_back((x, c, distance + 1));
            }
        }
    }
    writeln!(
        out,
        "{}",
        if min == 1_000_000_007 {
            -1
        } else {
            min as isize
        }
    )?;
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new(stdin().bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
