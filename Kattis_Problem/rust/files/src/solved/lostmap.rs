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

// Solution Code Here

#[derive(Clone)]
struct Edge {
    w: usize,
    to: isize,
}

impl Edge {
    fn new() -> Self {
        Self { w: INF, to: -1 }
    }
}

fn prims(g: Vec<Vec<usize>>) -> Option<(usize, Vec<(usize, usize)>)> {
    let n = g.len();
    let mut total_weight = 0;
    let mut selected = vec![false; n];
    let mut min_e = vec![Edge::new(); n];
    let mut tree = vec![];
    min_e[0].w = 0;
    for _ in 0..n {
        let mut v = -1;
        for j in 0..n {
            if !selected[j] && (v == -1 || min_e[j].w < min_e[v as usize].w) {
                v = j as isize;
            }
        }
        if min_e[v as usize].w == INF {
            return None;
        }
        selected[v as usize] = true;
        total_weight += min_e[v as usize].w;
        if min_e[v as usize].to != -1 {
            tree.push((v as usize, min_e[v as usize].to as usize))
        }
        for to in 0..n {
            if g[v as usize][to] < min_e[to].w {
                min_e[to] = Edge {
                    w: g[v as usize][to],
                    to: v,
                };
            }
        }
    }
    Some((total_weight, tree))
}

pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![0; n],
            size: vec![0; n],
        }
    }
    pub fn make_set(&mut self, n: usize) {
        self.parent[n] = n;
        self.size[n] = 1;
    }
    pub fn find(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            v
        } else {
            self.parent[v] = self.find(self.parent[v]);
            self.parent[v]
        }
    }
    pub fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a != b {
            if self.size[a] < self.size[b] {
                std::mem::swap(&mut a, &mut b);
            }
            self.parent[b] = a;
        }
    }
}

fn kruskal(g: Vec<Vec<usize>>) -> (usize, Vec<(usize, usize)>) {
    let n = g.len();
    let mut total_weight = 0;
    let mut tree = Vec::with_capacity(n);
    let mut edges = Vec::with_capacity(n * n);
    let mut dsu = DSU::new(n * n);
    for i in 0..n {
        dsu.make_set(i);
        for j in i + 1..n {
            edges.push((i, j, g[i][j]));
        }
    }
    edges.sort_by(|(_, _, a), (_, _, b)| a.cmp(&b));
    for (u, v, w) in edges {
        if dsu.find(u) != dsu.find(v) {
            total_weight += w;
            tree.push((u, v));
            dsu.union(u, v)
        }
    }
    (total_weight, tree)
}

#[allow(non_snake_case)]
fn main() -> Result<(), StopCode> {
    let mut scan = Scanner::new(stdin().bytes());
    let mut out = BufWriter::new(stdout());
    let n = scan.next::<Uint>()?;
    let g = (0..n)
        .map(|_| (0..n).flat_map(|_| scan.next::<Uint>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (_, tree) = kruskal(g);
    for (v, w) in tree {
        writeln!(out, "{} {}", v + 1, w + 1)?;
    }
    Ok(out.flush()?)
}
