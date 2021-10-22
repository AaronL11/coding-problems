
use std::{collections::HashMap, error, io, str};
// ******************* TOOLS ********************** //

#[allow(dead_code)]
fn input<T>() -> Result<T, Box<dyn error::Error>>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: 'static + error::Error
{
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let parsed = line.trim_end().parse::<T>()?;
    Ok(parsed)
}

#[allow(dead_code)]
fn vec_from_input<T>(delim: &str) -> Result<Vec<T>, Box<dyn error::Error>>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: 'static + error::Error
{
    let vec = input::<String>()?;
    let vec = vec.split(delim).map(|x| x.parse::<T>().unwrap());
    let vec: Vec<T> = vec.collect();
    Ok(vec)
}

#[allow(dead_code)]
fn get_tuple_input<A,B>() -> Result<(A,B), Box<dyn error::Error>>
where
    A: str::FromStr,
    <A as str::FromStr>::Err: 'static + error::Error,
    B: str::FromStr,
    <B as str::FromStr>::Err: 'static + error::Error
{
    let v = input::<String>()?;
    let v: Vec<_> = v.split(" ").collect();
    // eprintln!("{:#?}",v);
    Ok((v[0].parse::<A>()?, v[1].trim().parse::<B>()?))
}

// ********************** ACTUAL CODE *********************** //

fn main() -> Result<(), Box<dyn error::Error>>{
    Ok(())
}
