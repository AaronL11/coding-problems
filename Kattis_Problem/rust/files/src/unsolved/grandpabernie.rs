mod tools;
use tools::*;
use std::collections::HashMap;
use std::error;
use std::str;

// UnSolved TOO SLOW



fn main() -> Result<(), Box<dyn error::Error>>{
    let mut dates: HashMap<String,Vec<i32>> = HashMap::new();
    let mut cities: HashMap<String,Vec<i32>> = HashMap::new();
    let n = input::<i32>()?;
    for _ in 0..n {
        let (s,y) = get_tuple_input::<String,i32>()?;

        if cities.contains_key(&s) {
            let v = cities.get_mut(&s)
                .unwrap();
            v.push(y);
            v.sort();
        } else {
            cities.insert(s, vec![y]);
        }
    }
    let q = input::<i32>()?;
    let mut stack: Vec<i32> = Vec::new();
    for _ in 0..q {
        let (s,k) = get_tuple_input::<String,usize>()?;
        stack.push(cities.get(&s).unwrap()[k-1])
    }
    Ok(stack.iter().for_each(|i| println!("{}",i)))
}

#[cfg(tests)]
mod tests {
    fn grandpabernie_test_1() {}
}

