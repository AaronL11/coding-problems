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

#[allow(dead_code)]
const DP_SIZE: Int = 0;

use std::collections::HashSet;

fn generate(d: u16,n: u16, v: &mut HashSet<u8>) {
    if n>200 { return }
    match d {
        0 => {
            let n = n*10;
            if n <= 200 {
                v.insert(n as u8);
            }
            if n!=0 {
                generate(0,n,v);
            }
        }
        1|2|4|5 => {
            generate(d+1,n,v);
            generate(d+3,n,v);
            let n = n*10+d;
            if n <= 200 {
                v.insert(n as u8);
            }
            generate(d,n,v);
            generate(d+1,n,v);
            generate(d+3,n,v);
        }
        3|6 => {
            generate(d+3,n,v);
            let n = n*10+d;
            if n <= 200 {
                v.insert(n as u8);
            }
            generate(d,n,v);
            generate(d+3,n,v);
        }
        7 => {
            generate(8,n,v);
            let n = n*10+d;
            if n <= 200 {
                v.insert(n as u8);
            }
            generate(8,n,v);
        }
        8 => {
            generate(9,n,v);
            generate(0,n,v);
            let n = n*10+d;
            if n <= 200 {
                v.insert(n as u8);
            }
            generate(9,n,v);
            generate(0,n,v);
        }
        9 => {
            let n = n*10+d;
            if n <= 200 {
                v.insert(n as u8);
            }
            generate(9,n, v);
        }
        _ => panic!()
    }
}

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    let mut values = HashSet::with_capacity(200);
    generate(1,0,&mut values);
    for _ in 0..scan.next::<u8>()? {
        let k = scan.next::<u8>()?;
        writeln!(
            out,
            "{}",
            if values.contains(&k) {
                k
            } else {
                let mut res = 0;
                for i in 0..values.len() as u8 {
                    let (l,r) = if i > k {
                        (k,k+i)
                    } else {
                        (k-i,k+i)
                    };
                    if values.contains(&l) {
                        res = l;
                        break
                    } else if values.contains(&r) {
                        res = r;
                        break
                    }
                }
                res
            }
            )?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

