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
    use std::cmp::Ordering;
    let (N,t) = scan.take_tuple::<usize,u8>()?;
    match t {
        1 => writeln!(out,"{}",7)?,
        2 => writeln!(
            out,
            "{}",
            {
                let (a,b) = scan.take_tuple::<u32,u32>()?;
                match a.cmp(&b) {
                    Ordering::Greater => "Bigger",
                    Ordering::Equal => "Equal",
                    Ordering::Less => "Smaller"
                }
            }
            )?,
        3 => writeln!(
                out,
                "{}",
                {
                    let mut s = scan.take::<u32>(3)?;
                    s.sort();
                    s[1]
                }
            )?,
        4 => writeln!(
                out,
                "{}",
                (0..N).into_iter()
                    .map(|_| scan.next::<u64>().unwrap())
                    .sum::<u64>()
            )?,
        5 => writeln!(
                out,
                "{}",
                (0..N).into_iter()
                    .map(|_| scan.next::<u64>().unwrap())
                    .filter(|i| i%2==0)
                    .sum::<u64>()
            )?,
        6 => writeln!(
                out,
                "{}",
                unsafe {
                    str::from_utf8_unchecked(
                        &((0..N).into_iter()
                                .map(|_|(scan.next::<u32>().unwrap()%26) as u8 + 97)
                                .map(|i| dbg!(i))
                                .collect::<Vec<u8>>()
                        ))
                }
                )?,
        7 => writeln!(
                out,
                "{}",
                {
                    let mut s = "Cyclic";
                    let mut i = 0;
                    let mut mem = 0;
                    let A = scan.take::<u32>(N)?;
                    loop {
                        mem = i;
                        i = A[i as usize];
                        if (i as usize)>=N {
                            s = "Out";
                            break;
                        } else if (i as usize)==N-1 {
                            s = "Done";
                            break;
                        } else if A[i as usize]==mem {
                            s = "Cyclic";
                            break;
                        }
                    }
                    s
                }
            )?,
        _ => panic!()
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}


