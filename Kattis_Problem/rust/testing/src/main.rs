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

use std::collections::HashMap;

struct DP {
    memo: HashMap<Int,Int>
}

impl DP {
    fn new() -> Self {
        let mut memo = HashMap::new();
        memo.insert(0,0);
        memo.insert(1,1);
        memo.insert(2,1);
        Self { memo }
    }
    fn fib(&mut self, n: Int) -> Int {

        if let Some(&i) = self.memo.get(&n) {
            i
        } else if n%2==0 {
            let k = n/2;
            let a = if let Some(&a) = self.memo.get(&k) {
                a
            } else {
                let fib = self.fib(k) % MOD;
                self.memo.insert(k+1,fib);
                self.memo[&k]
            };
            let b = if let Some(&b) = self.memo.get(&(k-1)) {
                b
            } else {
                let fib = self.fib(k-1) % MOD;
                self.memo.insert(k-1,fib);
                self.memo[&(k-1)]
            };
            self.memo.insert(
                n,
                (a*(2*b+a))
                % MOD
            );
            self.memo[&n]
        } else {
            let k = (n+1)/2;
            let a = if let Some(&a) = self.memo.get(&k) {
                a
            } else {
                let fib = self.fib(k);
                self.memo.insert(k+1,fib);
                self.memo[&k]
            };
            let b = if let Some(&b) = self.memo.get(&(k-1)) {
                b
            } else {
                let fib = self.fib(k-1);
                self.memo.insert(k-1,fib);
                self.memo[&(k-1)]
            };
            self.memo.insert(
                n,
                (a*a + b*b)
                % MOD
            );
            self.memo[&n]
        }
    }
}

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    let mut dp = DP::new();
    for _ in 0..scan.next::<u8>()? {
        let (K,Y) = scan.take_tuple::<u8,Int>()?;
        writeln!(
            out,
            "{} {}",
            K,
            dp.fib(Y) % MOD
            )?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

