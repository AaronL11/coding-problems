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
    use std::collections::{HashMap,HashSet};
    use std::iter::FromIterator;
    let (N,t) = scan.take_tuple::<usize,u8>()?;
    match t {
        1 => {
            let set: HashSet<u32> = HashSet::from_iter(
                                                (0..N).into_iter()
                                                    .map(
                                                        |_|
                                                        scan.next::<u32>().unwrap())
                                                    );
            let mut res = "No";
            for i in &set {
                if set.contains(&(7777-*i)) {
                    res = "Yes";
                    break
                }
            }
            writeln!(out,"{}",res)?;
        },
        2 => {
            let set: HashSet<u32> = HashSet::from_iter(
                                                (0..N).into_iter()
                                                    .map(
                                                        |_|
                                                        scan.next::<u32>().unwrap())
                                                    );
            if set.len() == N {
                writeln!(out,"Unique")?;
            } else {
                writeln!(out,"Contains duplicate")?;
            }
        },
        3 => {
            let mut count: HashMap<usize,usize> = HashMap::new();
            let mut res = -1;
            for _ in 0..N {
                let i = scan.next::<usize>()?;
                if let Some(num) = count.get_mut(&i) {
                    *num += 1;
                    if *num > N/2 {
                        res = i as isize;
                        break
                    }
                } else {
                    count.insert(i,1);
                }
            }
            writeln!(out,"{}",res)?;
        },
        4 => {
            let mut A = scan.take::<u32>(N)?;
            A.sort();
            if N%2==1 {
                writeln!(out,"{}",A[N/2])?;
            } else {
                writeln!(out,"{} {}",A[N/2-1],A[N/2])?;
            }
        },
        5 => {
            let mut B = (0..N).into_iter()
                            .map(|_| scan.next::<u32>().unwrap())
                            .filter(|&i| 100<=i && i<=999)
                            .collect::<Vec<u32>>();
            B.sort();
            write!(out,"{}",B[0])?;
            for i in 1..B.len() {
                write!(out," {}",B[i])?;
            }
        },
        _ => panic!()
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}

