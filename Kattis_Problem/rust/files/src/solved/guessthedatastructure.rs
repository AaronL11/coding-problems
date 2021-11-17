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

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    use std::collections::{VecDeque,BinaryHeap};
    const SIZE: usize = 1000;
    let mut stack = Vec::with_capacity(SIZE);
    let mut prique = BinaryHeap::with_capacity(SIZE);
    let mut deque = VecDeque::with_capacity(SIZE);
    let mut buf = String::new();
    loop {
        stdin().lock().read_line(&mut buf)?;
        let n = buf.trim_end();
        if n=="" { break }
        let n = n.parse::<Int>().unwrap();
        let (mut s,mut p,mut d) = (true,true,true);
        for _ in 0..n {
            let (t,v) = scan.take_tuple::<u8,u8>()?;
            match t {
                1 => {
                    stack.push(v);
                    deque.push_back(v);
                    prique.push(v);
                }
                2 => {
                    if s {
                        s = if let Some(x) = stack.pop() {
                            if v==x { true } else { false }
                        } else {
                            false
                        }
                    }
                    if p {
                        p = if let Some(x) = prique.pop() {
                            if v==x { true } else { false }
                        } else {
                            false
                        }
                    }
                    if d {
                        d = if let Some(x) = deque.pop_front() {
                            if v==x { true } else { false }
                        } else {
                            false
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        if s && p || s && d || d && p {
            writeln!(out,"not sure")?;
        } else if s {
            writeln!(out,"stack")?;
        } else if p {
            writeln!(out,"priority queue")?;
        } else if d {
            writeln!(out,"queue")?;
        } else {
            writeln!(out,"impossible")?;
        }
        stack.clear();
        prique.clear();
        deque.clear();
        buf.clear()
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

