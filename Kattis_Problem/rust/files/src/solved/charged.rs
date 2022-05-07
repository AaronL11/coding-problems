#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    error,
    f64::consts::PI,
    fmt,
    fmt::{Display, Formatter},
    io,
    io::{stdin, stdout, BufRead, BufWriter, Bytes, Read, Stdin, Stdout, Write},
    str,
    str::FromStr,
};

type Int = isize;
type Uint = usize;
const MOD: Uint = 1_000_000_007;
// const INF: Int = isize::MAX;

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

const A: f64 = 1.0 / PI;
const B: f64 = A / PI;
const C: f64 = B / PI;

fn field(q: Int, x1: Uint, x0: Uint, y1: Uint, y0: Uint) -> f64 {
    q as f64
        / ((x1 as f64 - x0 as f64) * (x1 as f64 - x0 as f64)
            + (y1 as f64 - y0 as f64) * (y1 as f64 - y0 as f64))
            .sqrt()
}

fn field_char(v: f64) -> char {
    let va = v.abs();
    if v > 0. {
        if va > A {
            '0'
        } else if va > B {
            'O'
        } else if va > C {
            'o'
        } else {
            '.'
        }
    } else if v < 0. {
        if va > A {
            '%'
        } else if va > B {
            'X'
        } else if va > C {
            'x'
        } else {
            '.'
        }
    } else {
        '.'
    }
}

/*
 * Very simple O(nmq) solution.
 * Simply loop over the grid, and then sum all the potentials at that point.
 * To save space I just write directly to a buffer instead of a grid.
 */

#[allow(non_snake_case)]
fn solve<R>(mut scan: Scanner<R>, mut out: BufWriter<Stdout>) -> Result<(), StopCode>
where
    R: Read,
{
    let (n, m, q) = scan.take_tuple3::<Uint, Uint, Uint>()?;
    let charges = (0..q)
        .map(|_| {
            let (x, y, c) = scan.take_tuple3::<Uint, Uint, char>().unwrap();
            (x, y, if c == '+' { 1 } else { -1 })
        })
        .collect::<HashSet<_>>();
    for y1 in 1..=m {
        for x1 in 1..=n {
            if charges.contains(&(x1, y1, 1)) {
                write!(out, "+")?;
            } else if charges.contains(&(x1, y1, -1)) {
                write!(out, "-")?;
            } else {
                write!(
                    out,
                    "{}",
                    field_char(
                        charges
                            .iter()
                            .map(|(x0, y0, q)| field(*q, x1, *x0, y1, *y0))
                            .sum::<f64>(),
                    )
                )?;
            }
        }
        write!(out, "\n")?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new(stdin().bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
