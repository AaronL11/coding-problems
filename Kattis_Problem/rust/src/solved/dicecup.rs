use std::{io,cmp::{min,max}};
use std::io::BufRead;

#[allow(non_snake_case,dead_code)]
fn main() {
   let mut inputs = String::new();
   io::stdin().lock().read_line(&mut inputs).unwrap();
   let nm: Vec<u8> = inputs.trim()
                    .split(' ')
                    .map(|c| c.parse::<u8>().unwrap())
                    .collect();
    let (N,M): (u8,u8) = (nm[0],nm[1]);
    if N==M {
        println!("{}",N+1) 
    } else {
        (min(N,M)+1..=max(N,M)+1).into_iter()
                                .for_each(|i| println!("{}",i))
    }
}

#[allow(non_snake_case,dead_code)]
fn test_fn(N: u8, M: u8) -> String {
    let mut output = String::new();
    if N==M {
        output.push_str(format!("{}",N+1).as_str());
    } else {
        (min(N,M)+1..=max(N,M)+1).into_iter()
                                .for_each(|i| output.push_str(format!("{}\n",i).as_str()))
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dicecup_test1() {
        assert_eq!(
            test_fn(6,6),
            String::from("7")
        )
    }

    #[test]
    fn dicecup_test2() {
        assert_eq!(
            test_fn(6,4),
            String::from("5\n6\n7\n")
        )
    }

    #[test]
    fn dicecup_test3() {
        assert_eq!(
            test_fn(12,20),
            String::from("13\n14\n15\n16\n17\n18\n19\n20\n21\n")
        )
    }
}

