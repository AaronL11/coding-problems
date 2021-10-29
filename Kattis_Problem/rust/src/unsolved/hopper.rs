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
        loop {
            if let Some(input) = self.iterator.next() {
                return input.parse::<T>().ok().ok_or(StopCode)
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

#[derive(Debug)]
struct mmoize(Vec<u16>);

impl mmoize {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(0);
        }
        Self(v)
    }
    fn path(&mut self, i: usize, D: usize, M: i32, v: &Vec<i32>) -> u16 {
        if dbg!(self.0[i]!=0) {
            return self.0[i];
        }
        let mut sum = 0;
        let n = v.len();
        for d in 1..D {
            dbg!(&d);
            if dbg!(i+d <=n-1) {
                let (a,b) = dbg!((v[i],v[i+d]));
                if dbg!((a-b).abs() <= M) {
                    sum += 1 + self.path(i+d,D,M,v);
                }
            }
            if i>d {
                let (a,b) = dbg!((v[i],v[i-d]));
                if dbg!((a-b).abs() <= M) {
                    sum += 1 + self.path(i-d,D,M,v);
                }
            }
        }
        self.0[i] = sum;
        dbg!(&self);
        dbg!(&sum);
        sum
    }
}


#[allow(non_snake_case,unused_variables)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    use std::collections::{HashSet,VecDeque};

    let (n,D,M) = scan.take_tuple3::<usize,u8,i32>()?;
    let A = scan.take::<i32>(n)?;
    let mut v: Vec<HashSet<i32>> = Vec::with_capacity(n);

    let mut sum = 0;
    let mut m: Vec<(Vec<usize>,Vec<usize>)> = Vec::with_capacity(n);

    for i in 0..n {
        let mut r = Vec::new();
        let mut l = Vec::new();
        for d in 1..D as usize {
            if i+d < n {
                let (a,b) = (A[i],A[i+d]);
                if (b-a).abs() <= M {
                    r.push(i+d);
                }
            }
            if i>d {
                let (a,b) = (A[i],A[i-d]);
                if (b-a).abs() <= M {
                    l.push(i-d);
                }
            }
        }
        m.push((l,r));
    }
    writeln!(
        out,
        "{:#?}",
        m
        )?;
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}



