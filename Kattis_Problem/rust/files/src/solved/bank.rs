use std::{io,error};
use std::collections::{HashMap,BinaryHeap};
use std::io::BufRead;

#[allow(unused_variables,dead_code)]
fn main() -> Result<(),Box<dyn error::Error>> {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;
    let args = buf.trim()
              .split(' ')
              .map(str::parse::<u64>)
              .map(Result::unwrap)
              .collect::<Vec<_>>();
    let (n,t) = (args[0],args[1]);
    let mut people: HashMap<u64,Vec<u64>> = HashMap::new();
    buf.clear();
    for _ in 0..n {
        io::stdin().lock().read_line(&mut buf)?;
        let args = buf.trim()
                        .split(' ')
                        .map(str::parse::<u64>)
                        .map(Result::unwrap)
                        .collect::<Vec<_>>();
        let (m,w) = (args[0],args[1]);
        if people.contains_key(&w) {
            people.get_mut(&w).unwrap().push(m);
        } else {
            people.insert(w,vec![m]);
        }
    }

    let mut q = BinaryHeap::new();
    let mut total = 0;
    for time in (0..t).rev() {
        if people.contains_key(&time) {
            for cash in people.get(&time).unwrap() {
                q.push(cash)
            }
        }
        if !q.is_empty() {
            total += q.pop().unwrap();
        }
    }

    Ok(println!("{}",total))
}


