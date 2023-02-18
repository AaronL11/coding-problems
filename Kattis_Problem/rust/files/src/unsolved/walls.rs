#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    error, fmt,
    fmt::{Display, Formatter},
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
const INF: usize = 1_000_000_000;
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

fn eval(r: Int, c: (Int, Int), walls: &[(Int, Int)]) -> Uint {
    let (c1, c2) = c;
    let mut tot = 0;
    for (l, w) in walls {
        if (l - c1) * (l - c1) + (w - c2) * (w - c2) <= r * r {
            tot += 1;
        }
    }
    tot
}

fn solve(cranes: &Vec<(Int, Int)>, walls: &mut [(Int, Int); 4], r: Int) -> Option<Uint> {
    let n = cranes.len();

    for i in 0..n {
        let mut tot = 0;
        let mut track = [0, 0, 0, 0];
        let (a, b) = cranes[i];
        for x in 0..4 {
            let (l, w) = walls[x];
            if (l - a) * (l - a) + (w - b) * (w - b) <= r * r {
                track[x] = 1;
                tot += 1;
            }
        }
        if tot == 4 {
            return Some(1);
        }
    }

    for i in 0..n {
        let mut tot = 0;
        let mut track = [0, 0, 0, 0];
        let (a, b) = cranes[i];
        for x in 0..4 {
            let (l, w) = walls[x];
            if (l - a) * (l - a) + (w - b) * (w - b) <= r * r {
                track[x] = 1;
                tot += 1;
            }
        }
        for j in i + 1..n {
            let (c, d) = cranes[j];
            for x in 0..4 {
                if track[x] == 1 {
                    continue;
                }
                let (l, w) = walls[x];
                if (l - c) * (l - c) + (w - d) * (w - d) <= r * r {
                    track[x] = 1;
                    tot += 1;
                }
            }
            if tot == 4 {
                return Some(2);
            }
        }
    }

    for i in 0..n {
        let mut tot = 0;
        let mut track = [0, 0, 0, 0];
        let (a, b) = cranes[i];
        for x in 0..4 {
            let (l, w) = walls[x];
            if (l - a) * (l - a) + (w - b) * (w - b) <= r * r {
                track[x] = 1;
                tot += 1;
            }
        }
        for j in i + 1..n {
            let (c, d) = cranes[j];
            for x in 0..4 {
                if track[x] == 1 {
                    continue;
                }
                let (l, w) = walls[x];
                if (l - c) * (l - c) + (w - d) * (w - d) <= r * r {
                    track[x] = 1;
                    tot += 1;
                }
            }
            for k in j + 1..n {
                let (e, f) = cranes[k];
                for x in 0..4 {
                    if track[x] == 1 {
                        continue;
                    }
                    let (l, w) = walls[x];
                    if (l - e) * (l - e) + (w - f) * (w - f) <= r * r {
                        track[x] = 1;
                        tot += 1;
                    }
                    if tot == 4 {
                        return Some(3);
                    }
                }
            }
        }
    }

    for i in 0..n {
        let mut tot = 0;
        let mut track = [0, 0, 0, 0];
        let (a, b) = cranes[i];
        for x in 0..4 {
            let (l, w) = walls[x];
            if (l - a) * (l - a) + (w - b) * (w - b) <= r * r {
                track[x] = 1;
                tot += 1;
            }
        }
        for j in i + 1..n {
            let (c, d) = cranes[j];
            for x in 0..4 {
                if track[x] == 1 {
                    continue;
                }
                let (l, w) = walls[x];
                if (l - c) * (l - c) + (w - d) * (w - d) <= r * r {
                    track[x] = 1;
                    tot += 1;
                }
            }
            for k in j + 1..n {
                let (e, f) = cranes[k];
                for x in 0..4 {
                    if track[x] == 1 {
                        continue;
                    }
                    let (l, w) = walls[x];
                    if (l - e) * (l - e) + (w - f) * (w - f) <= r * r {
                        track[x] = 1;
                        tot += 1;
                    }
                }
                for w in k + 1..n {
                    let (g, h) = cranes[w];
                    for x in 0..4 {
                        if track[x] == 1 {
                            continue;
                        }
                        let (l, w) = walls[x];
                        if (l - g) * (l - g) + (w - h) * (w - h) <= r * r {
                            track[x] = 1;
                            tot += 1;
                        }
                    }
                    if tot == 4 {
                        return Some(4);
                    }
                }
            }
        }
    }
    None
}

#[allow(non_snake_case)]
fn main() -> Result<(), StopCode> {
    let mut scan = Scanner::new(stdin().bytes());
    let mut out = BufWriter::new(stdout());
    let (l, w) = scan.take_tuple::<Int, Int>()?;
    let mut walls = [(-l / 2, 0), (l / 2, 0), (0, -w / 2), (0, w / 2)];
    let (n, r) = scan.take_tuple::<Uint, Int>()?;
    let cranes = (0..n)
        .flat_map(|_| scan.take_tuple::<Int, Int>())
        .collect::<Vec<_>>();
    if let Some(ans) = solve(&cranes, &mut walls, r) {
        writeln!(out, "{}", ans)?;
    } else {
        writeln!(out, "Impossible")?;
    }
    Ok(out.flush()?)
}
