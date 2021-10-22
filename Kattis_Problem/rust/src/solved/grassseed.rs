use std::{io};
use std::io::BufRead;

#[allow(non_snake_case,dead_code)]
fn main() -> io::Result<()> {
    let mut inputs = String::new();
    let io = io::stdin();
    io.lock().read_line(&mut inputs)?;
    let C = inputs.trim().parse::<f64>().unwrap();
    inputs.clear();
    io.lock().read_line(&mut inputs)?;
    let L = inputs.trim().parse::<u8>().unwrap();
    let x = (0..L).into_iter()
        .map(
            |_| {
                inputs.clear();
                io.lock().read_line(&mut inputs).unwrap();
                inputs.trim()
                    .split(' ')
                    .map(|c| c.parse::<f64>().unwrap())
                    .product::<f64>()
            }
        )
        .sum::<f64>()*C;
    println!("{}",x);
    Ok(())
}

#[allow(non_snake_case,dead_code)]
fn test_fn(io: String) -> f64 {
    let io = io.split('\n').collect::<Vec<_>>();
    let C = io[0].parse::<f64>().unwrap();
    let L = io[1].parse::<u8>().unwrap();
    let x = (2..L+2).into_iter()
        .map(
            |i| {
                io[i as usize].trim()
                    .split(' ')
                    .map(|c| c.parse::<f64>().unwrap())
                    .product::<f64>()
            }
        )
        .sum::<f64>()*C;
    x
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grassseed_test1() {
        assert_eq!(
            test_fn(String::from("2\n3\n2 3\n4 5\n5 6")),
            112.0000000
            )
    }

    #[test]
    fn grassseed_test2() {
        assert_eq!(
            test_fn(String::from("0.75\n2\n2 3.333\n3.41 4.567")),
           16.6796025
        )
    }
}

