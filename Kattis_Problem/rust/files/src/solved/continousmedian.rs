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

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;
    let mut less = BinaryHeap::new();
    let mut greater = BinaryHeap::new();
    let T = scan.next::<u8>()?;
    for _ in 0..T {
        let N = scan.next::<usize>()?;
        let mut med = [scan.next::<i64>()?,0];
        let mut sum = med[0];
        for i in 2..=N {
            let cmp = i%2==0;
            let n = scan.next::<i64>()?;
            match n.cmp(&med[0]) {
                Ordering::Less => {
                    sum += match cmp {
                        true => {
                            less.push(n);
                            med = [less.pop().unwrap(),med[0]];
                            (med[0]+med[1])/2
                        }
                        false => {
                            greater.push(-med[1]);
                            less.push(n);
                            med = [med[0],0];
                            med[0]
                        }
                    }
                }
                Ordering::Equal => {
                    sum += match cmp {
                        true => {
                            med = [med[0],n];
                            (med[0]+med[1])/2
                        }
                        false => {
                            greater.push(-med[1]);
                            less.push(n);
                            med = [med[0],0];
                            med[0]
                        }
                    }
                }
                Ordering::Greater => {
                    sum += match cmp {
                        true => {
                            greater.push(-n);
                            med = [med[0],-greater.pop().unwrap()];
                            (med[0]+med[1])/2
                        }
                        false => {
                            med = if n<=med[1] {
                                less.push(med[0]);
                                greater.push(-med[1]);
                                [n,0]
                            } else {
                                less.push(med[0]);
                                greater.push(-n);
                                [med[1],0]
                            };
                            med[0]
                        }
                    }
                }
            }
        }
        less.clear();
        greater.clear();
        writeln!(out,"{}",sum)?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

