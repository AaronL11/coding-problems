use io::{stdin, stdout, BufRead, BufWriter, Stdin, Stdout, Write};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io;
use std::{error, fmt, str};

#[derive(Debug, Default)]
struct StopCode;

impl error::Error for StopCode {}

impl Display for StopCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Code failed to execute")
    }
}

impl From<io::Error> for StopCode {
    fn from(_: io::Error) -> StopCode {
        StopCode
    }
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
            iterator: "".split_ascii_whitespace(),
        }
    }
    fn next<T: str::FromStr>(&mut self) -> Result<T, StopCode> {
        loop {
            if let Some(input) = self.iterator.next() {
                return input.parse::<T>().ok().ok_or(StopCode);
            }
            self.buffer.clear();
            self.scanin.lock().read_until(b'\n', &mut self.buffer)?;
            self.iterator = unsafe {
                let slice = str::from_utf8_unchecked(&self.buffer);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }
    fn take<T: str::FromStr>(&mut self, n: usize) -> Result<Vec<T>, StopCode> {
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
    fn take_tuple<T, V>(&mut self) -> Result<(T, V), StopCode>
    where
        T: str::FromStr,
        V: str::FromStr,
    {
        Ok((self.next::<T>()?, self.next::<V>()?))
    }
    fn take_tuple3<T, V, U>(&mut self) -> Result<(T, V, U), StopCode>
    where
        T: str::FromStr,
        V: str::FromStr,
        U: str::FromStr,
    {
        Ok((self.next::<T>()?, self.next::<V>()?, self.next::<U>()?))
    }
}

#[allow(non_snake_case)]
fn solve(mut scan: Scanner, mut out: BufWriter<Stdout>) -> Result<(), StopCode> {
    let t = scan.next::<usize>()?;
    let mut king = HashSet::new();
    let mut queen = HashSet::new();
    for _ in 0..t {
        king.clear();
        queen.clear();
        let (dx, dy) = scan.take_tuple::<i32, i32>()?;
        let (kx, ky) = scan.take_tuple::<i32, i32>()?;
        let (qx, qy) = scan.take_tuple::<i32, i32>()?;
        let diffs = [
            (dx, dy),
            (dy, dx),
            (dx, -dy),
            (-dx, dy),
            (dy, -dx),
            (-dy, dx),
            (-dx, -dy),
            (-dy, -dx),
        ];
        for (x, y) in diffs {
            king.insert((kx + x, ky + y));
            queen.insert((qx + x, qy + y));
        }
        writeln!(out, "{}", king.intersection(&queen).count())?;
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
