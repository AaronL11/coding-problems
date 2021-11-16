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
    use std::cmp::min;
    let P = scan.get_str()?
                .bytes()
                .map(|b| if b==b'R' { (1,-1) } else { (-1,1) })
                .collect::<Vec<(isize,isize)>>();
    let (mut bb,mut br) = (P[0].1,P[0].0);
    let (mut sb,mut sr) = (P[0].1,P[0].0);
    let (mut lb,mut rb) = (0,0);
    let (mut lr,mut rr) = (0,0);
    let (mut mlb,mut mrb) = (0,0);
    let (mut mlr,mut mrr) = (0,0);
    for i in 1..P.len() {
        let (r,b) = P[i];
        if r > r + sr {
            sr = r;
            lr = i;
        } else {
            sr += r;
        }
        rr = i;
        if sr > br {
            br = sr;
            mlr = lr;
            mrr = rr;
        }
        if b > b + sb {
            sb = b;
            lb = i;
        } else {
            sb += b;
        }
        rb = i;
        if sb > bb {
            bb = sb;
            mlb = lb;
            mrb = rb;
        }
    }
    if bb > br {
        writeln!(out,"{} {}",mlb+1,mrb+1)?;
    } else if bb == br {
        writeln!(out,"{} {}",min(mlb,mlr)+1,min(mrb,mrr)+1)?;
    } else {
        writeln!(out,"{} {}",mlr+1,mrr+1)?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}


