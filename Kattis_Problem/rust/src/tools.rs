use std::{io,io::BufRead,str,error};

#[allow(dead_code)]
pub fn input<T>() -> Result<T, Box<dyn error::Error>>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: 'static + error::Error
{
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    let parsed = line.trim_end().parse::<T>()?;
    Ok(parsed)
}

#[allow(dead_code)]
pub fn vec_from_input<T>(delim: &str) -> Result<Vec<T>, Box<dyn error::Error>>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: 'static + error::Error
{
    let vec = input::<String>()?;
    let vec = (&vec).split(delim).map(|x| x.parse::<T>().unwrap());
    let vec: Vec<T> = vec.collect();
    Ok(vec)
}

#[allow(dead_code)]
pub fn get_tuple_input<A,B,const SIZE: usize>() -> Result<(A,B), Box<dyn error::Error>>
where
    A: str::FromStr,
    <A as str::FromStr>::Err: 'static + error::Error,
    B: str::FromStr,
    <B as str::FromStr>::Err: 'static + error::Error
{
    let v = input::<String>()?;
    let v: Vec<_> = v.split(" ").collect();
    Ok((v[0].parse::<A>()?, v[0].parse::<B>()?))
}
