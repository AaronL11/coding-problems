#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
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

struct Fenwick {
    fenwick: [Uint; 200_000],
    gems: Vec<Uint>,
    values: [Uint; 6],
}

impl Fenwick {
    fn new(gems: Vec<Uint>, values: [Uint; 6]) -> Self {
        let mut fenwick = [0; 200_000];
        for i in 0..gems.len() {
            fenwick[i] = values[gems[i]];
        }
        Self {
            fenwick,
            gems,
            values,
        }
    }
    fn dbg(&self, n: Uint) {
        dbg!(&self.gems[..n], &self.values);
    }
    fn increment(&mut self, idx: Int, inc: Int) {
        let mut pos = idx;
        while pos < self.fenwick.len() as Int {
            self.fenwick[pos as Uint] += self.values[self.gems[pos as Uint]];
            pos += pos & (-pos);
        }
    }
    fn get_sum(&self, idx: Int) -> Int {
        let mut i = idx;
        let mut sum = 0;
        while i > 0 {
            sum += self.values[self.gems[i as Uint]];
            i -= i & (-i);
        }
        sum as Int
    }
    fn replace(&mut self, k: Uint, p: Uint) {
        self.gems[k] = p;
    }
    fn revalue(&mut self, p: Uint, v: Uint) {
        self.values[p] = v;
    }
    fn range(&mut self, l: Int, r: Int) -> Int {
        self.get_sum(r) - self.get_sum(l - 1)
    }
}

#[allow(non_snake_case)]
fn solve<R>(mut scan: Scanner<R>, mut out: BufWriter<Stdout>) -> Result<(), StopCode>
where
    R: Read,
{
    let mut values = [0; 6];
    let (N, Q) = scan.take_tuple::<Uint, Uint>()?;
    let mut gems = vec![0; N];
    for i in 0..6 {
        values[i] = scan.next::<Uint>()?;
    }
    for (i, byte) in scan.get_str()?.bytes().enumerate() {
        gems[i] = byte as Uint - 49;
    }
    let mut fenwick = Fenwick::new(gems, values);
    fenwick.dbg(N);
    for _ in 0..Q {
        let n = scan.next::<u8>()?;
        match n {
            1 => fenwick.replace(scan.next::<Uint>()?, scan.next::<Uint>()? - 1),
            2 => fenwick.revalue(scan.next::<Uint>()? - 1, scan.next::<Uint>()?),
            _ => {
                writeln!(
                    out,
                    "{}",
                    fenwick.range(scan.next::<Int>()? - 1, scan.next::<Int>()? - 1)
                )?;
            }
        }
        dbg!(n);
        fenwick.dbg(N);
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new(stdin().bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
