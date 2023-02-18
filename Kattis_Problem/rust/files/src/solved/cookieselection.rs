#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    error, fmt,
    fmt::{Display, Formatter},
    io,
    io::{stdin, stdout, BufRead, BufWriter, Bytes, Read, Stdin, Stdout, Write},
    iter::FromIterator,
    str,
    str::FromStr,
};

// Constants

type Int = isize;
type Uint = usize;
const MOD: Uint = 1_000_000_007;
// const INF: Int = isize::MAX;

// Error handling

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

// Scanner code

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
    fn flush(&mut self) {
        self.buffer.clear()
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
    fn take_into<T: FromStr, V: FromIterator<T>>(&mut self, n: usize) -> V {
        (0..n).flat_map(|_| self.next::<T>()).collect::<V>()
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

// Solution Code Here

#[allow(non_snake_case)]
fn main() -> Result<(), StopCode> {
    let mut scan = Scanner::new(stdin().bytes());
    let mut out = BufWriter::new(stdout());
    let mut h1: BinaryHeap<Int> = BinaryHeap::new();
    let mut mid = 0;
    let mut h2: BinaryHeap<Int> = BinaryHeap::new();
    let mut c = 0;
    loop {
        let y = scan.get_str();
        if let Err(_) = y {
            break;
        }
        if let Ok(n) = y?.parse::<Int>() {
            match c {
                0 => mid = n,
                1 => {
                    if n <= mid {
                        h1.push(n);
                    } else {
                        h1.push(mid);
                        mid = n;
                    }
                }
                2 => {
                    if n <= mid {
                        let x = *h1.peek().unwrap();
                        if n <= x {
                            h2.push(-mid);
                            let t = h1.pop().unwrap();
                            h1.push(n);
                            mid = t;
                        } else {
                            h2.push(-mid);
                            mid = n;
                        }
                    } else {
                        h2.push(-n);
                    }
                }
                _ => {
                    if n <= mid {
                        let x = *h1.peek().unwrap();
                        if c % 2 == 0 {
                            if n >= x {
                                h2.push(-mid);
                                mid = n;
                            } else if n < x {
                                let t = h1.pop().unwrap();
                                h1.push(n);
                                h2.push(-mid);
                                mid = t;
                            }
                        } else {
                            h1.push(n);
                        }
                    } else if n > mid {
                        let y = -*h2.peek().unwrap();
                        if c % 2 == 0 {
                            h2.push(-n);
                        } else {
                            if n < y {
                                h1.push(mid);
                                mid = n;
                            } else if n >= y {
                                let t = -h2.pop().unwrap();
                                h1.push(mid);
                                h2.push(-n);
                                mid = t;
                            }
                        }
                    }
                }
            }
            c += 1;
        } else {
            writeln!(out, "{}", mid)?;
            if c % 2 == 0 {
                mid = if let Some(n) = h1.pop() { n } else { 0 };
            } else {
                mid = if let Some(n) = h2.pop() { -n } else { 0 };
            }
            c -= 1;
        }
    }
    Ok(out.flush()?)
}
