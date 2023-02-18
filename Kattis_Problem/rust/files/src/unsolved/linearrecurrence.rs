#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    error, fmt,
    fmt::{Display, Formatter},
    hash::Hash,
    io,
    io::{stdin, stdout, BufRead, BufWriter, Bytes, Read, Stdin, Stdout, Write},
    iter,
    iter::FromIterator,
    str,
    str::FromStr,
};

// Constants

type Int = isize;
type Uint = usize;
const EPSILON: f64 = 1e-5;
const INF: isize = 1_000_000_000;
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

// Solution Code

type Matrix = Vec<Vec<isize>>;
type Vector = Vec<isize>;

const N: usize = 46;

fn id() -> Matrix {
    (0..N)
        .map(|i| {
            (0..N)
                .map(|j| if i == j { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[allow(non_snake_case)]
fn mat_mul(A: &Matrix, B: &Matrix, m: isize) -> Matrix {
    (0..N)
        .map(|i| {
            (0..N)
                .map(|j| (0..N).map(|k| (A[i][k] * B[k][j]) % m).sum::<Int>() % m)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[allow(non_snake_case)]
fn mat_(A: &Matrix, v: &Vector, m: isize) -> Vector {
    (0..N)
        .map(|i| (0..N).map(|j| (A[i][j] * v[j]) % m).sum::<Int>() % m)
        .collect::<Vec<_>>()
}

#[allow(non_snake_case)]
fn mat_vec(A: &Matrix, v: &Vector, m: isize) -> isize {
    (0..N).fold(0, |sum, i| (sum + A[1][i] * v[i]) % m) % m
}

#[allow(non_snake_case)]
fn mat_exp(M: &Matrix, pow: usize, m: isize) -> Matrix {
    if pow == 0 {
        id()
    } else if pow == 1 {
        M.clone()
    } else {
        let A = mat_exp(M, pow / 2, m);
        if pow % 2 == 0 {
            mat_mul(&A, &A, m)
        } else {
            mat_mul(&M, &mat_mul(&A, &A, m), m)
        }
    }
}

#[allow(non_snake_case)]
fn main() -> Result<(), StopCode> {
    let mut scan = Scanner::new(stdin().bytes());
    let mut out = BufWriter::new(stdout());
    let n = scan.next::<Uint>()?;
    let mut M = vec![vec![0; N]; N];
    M[0][0] = 1;
    for i in 0..n + 1 {
        M[1][i] = scan.next::<Int>()?;
    }
    for i in 0..n {
        M[2 + i][i + 1] = 1;
    }
    let mut v = vec![0; N];
    v[0] = 1;
    for i in 0..n {
        v[n - i] = scan.next::<Int>()?;
    }
    let Q = scan.next::<Uint>()?;
    for _ in 0..Q {
        let (t, m) = scan.take_tuple::<Uint, Int>()?;
        if m == 1 {
            writeln!(out, "0")?;
        } else if t < n {
            writeln!(out, "{}", v[t] % m)?;
        } else if t == n {
            let mv = mat_vec(&M, &v, m);
            writeln!(out, "{}", mv)?;
        } else {
            let Mt = mat_exp(&M, t, m);
            let mv = mat_vec(&Mt, &v, m);
            writeln!(out, "{}", mv)?;
        }
    }
    Ok(out.flush()?)
}
