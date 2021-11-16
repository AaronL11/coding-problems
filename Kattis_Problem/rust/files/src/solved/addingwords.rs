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

#[allow(non_snake_case)]
fn solve(
    mut scan: Scanner,
    mut out: BufWriter<Stdout>
) -> Result<(), StopCode> {
    use std::collections::HashMap;
    let mut vars: HashMap<String,i32> = HashMap::with_capacity(2000);
    let mut buf = Vec::new();
    let stdin = stdin();
    let mut stdin = stdin.lock();
    loop {
        stdin.read_until(b'\n',&mut buf)?;
        buf.pop();
        if buf.is_empty() { break }
        let v = &buf[..].split(|&b| b==b' ')
                        .map(|b| unsafe { str::from_utf8_unchecked(b) }.to_owned())
                        .collect::<Vec<String>>();
        match &v[0][..] {
            "def" => {
                vars.insert(v[1].to_owned(),v[2].parse::<i32>().unwrap());
            }
            "calc" => {
                let mut good = true;
                let mut i = 3;
                let mut sum = 0;
                if let Some(num) = vars.get(&v[1]) {
                    sum = *num;
                } else {
                    good = false;
                }
                while good && i<v.len() {
                    if let Some(num) = vars.get(&v[i]){
                        if v[i-1] == "+" {
                            sum += *num;
                        } else {
                            sum -= *num;
                        }
                    } else {
                        good = false;
                        break
                    }
                    i += 2;
                }
                write!(out,"{}",v[1])?;
                for s in &v[2..] {
                    write!(out," {}",s)?;
                }
                if good {
                    for (k,&v) in vars.iter() {
                        good = false;
                        if sum == v {
                            writeln!(out," {}",k)?;
                            good = true;
                            break
                        }
                    }
                }
                if !good {
                    writeln!(out," unknown")?;
                }
            }
            "clear" => vars.clear(),
            _ => panic!()
        }
        buf.clear();
    }
    Ok(out.flush()?)
}

fn main() -> Result<(), StopCode> {
    let scan = Scanner::new();
    let out = BufWriter::new(stdout());
    solve(scan,out)
}


