use std::{str,error,fmt};
use std::io;
use io::{stdin,Stdin,BufRead,stdout,Stdout,BufWriter,Write};
use std::fmt::{Display,Formatter};

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
    let mut dp = vec![[0;2001];2001];
    let mut VW = [(0,0);2000];
    let mut buf = String::with_capacity(4005);
    loop {
        stdin().lock().read_line(&mut buf)?;
        dbg!(&buf);
        if buf.is_empty() || buf.eq("\n"){ break }
        let t = &buf[..].trim()
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
        buf.clear();
        let (C,n) = (t[0],t[1]);
        // O(n)
        for i in 0..n {
            VW[i] = scan.take_tuple::<usize,usize>()?;
        }
        // O(n^2)
        for v in 1..=n {
            for w in 1..=C {
                let (pv,pw) = VW[v-1];
                dp[v][w] = dp[v-1][w];
                if w>=pw && dp[v][w] < dp[v-1][w-pw]+pv {
                    dp[v][w] = dp[v-1][w-pw]+pv;
                }
            }
        }
        let mut S = Vec::with_capacity(n);
        let (mut i,mut res,mut w) = (n,dp[n][C],C);
        // O(n)
        while i>0 && res>0 {
            if res == dp[i-1][w] {
                continue
            } else {
                S.push(i-1);
            }
            let (v,wi) = VW[i-1];
            res -= v;
            w -= wi;
            i -= 1;
        }
        writeln!(out,"{}",S.len())?;
        write!(out,"{}",S[S.len()-1])?;
        for i in (0..S.len()-1).rev() {
            write!(out," {}",S[i])?;
        }
        write!(out,"\n")?;
        out.flush()?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

