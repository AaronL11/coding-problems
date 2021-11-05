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

fn two(scan: &mut Scanner) -> Result<&str,StopCode> {
    let (a,b) = scan.take_tuple::<u32,u32>()?;
    if a>b {
        Ok("Bigger")
    } else if a<b {
        Ok("Smaller")
    } else {
        Ok("Equal")
    }
}

fn three(scan: &mut Scanner) -> Result<u32,StopCode> {
    let (a,b,c) = scan.take_tuple3::<u32,u32,u32>()?;
    let mut t = [a,b,c];
    t.sort();
    Ok(t[1])
}

#[allow(unused_assignments)]
fn seven(scan: &mut Scanner, n: usize) -> Result<&str,StopCode> {
    use std::collections::HashSet;
    let mut result = "Cyclic";
    let mut used = HashSet::new();
    let v = scan.take::<usize>(n)?;
    let mut i = 0_usize;
    used.insert(0_usize);
    loop {
        i = v[i];
        if used.contains(&i) {
            result = "Cyclic";
            break
        } else if i>=n {
            result = "Out";
            break
        } else if i==n-1 {
            result = "Done";
            break
        }
        used.insert(i);
    }
    Ok(result)
}

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    let (N,t) = scan.take_tuple::<usize,u8>()?;
    let result = match t {
        1 => String::from("7"),
        2 => String::from(two(&mut scan)?),
        3 => three(&mut scan)?.to_string(),
        4 => scan.take::<u64>(N)?
                    .iter()
                    .sum::<u64>().to_string(),
        5 => scan.take::<u64>(N)?.iter()
                .filter(|&i| *i%2==0)
                .sum::<u64>().to_string(),
        6 => unsafe {String::from(str::from_utf8_unchecked(
                &scan.take::<u32>(N)?
                    .iter()
                    .map(|&i| (i%26) as u8 + 97)
                    .collect::<Vec<u8>>()))},
        7 => String::from(seven(&mut scan, N)?),
        _ => panic!()
    };
    writeln!(out,"{}",result)?;
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}


