use std::{io,cmp};
use std::io::BufRead;

#[allow(non_snake_case,dead_code)]
fn main() {
    let stdin = io::stdin();
    let pos = stdin.lock()
                    .split(b' ')
                    .map(|s| String::from_utf8(s.unwrap()).unwrap())
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>();
    let (sk,ek) = ((pos[0],pos[1]),(pos[4],pos[5]));
    let (so,eo) = ((pos[2],pos[3]),(pos[6],pos[7]));
    let sr = ((so.0-sk.0).powi(2)+(so.1-sk.1).powi(2)).sqrt();
    let er = ((eo.0-ek.0).powi(2)+(eo.1-ek.1).powi(2)).sqrt();
    println!("{:.10}",if sr>er { sr } else { er })
}

#[allow(non_snake_case,dead_code)]
fn test_fn(input: &[u8]) -> f64 {
    let cursor = io::Cursor::new(input);
    let pos = cursor.split(b' ')
                    .map(|s| String::from_utf8(s.unwrap()).unwrap())
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>();
    let (_sk,ek) = ((pos[0],pos[1]),(pos[4],pos[5]));
    let (_so,eo) = ((pos[2],pos[3]),(pos[6],pos[7]));
    let i = 1e10;
    (i*((eo.0-ek.0).powi(2)+(eo.1-ek.1).powi(2)).sqrt()).round()/i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jointjogjams_test1() {
        assert_eq!(
            test_fn(b"0 0 0 0 1 1 2 2"),
            1.4142135624
            )
    }

    #[test]
    fn jointjogjams_test2() {
        assert_eq!(
            test_fn(b"0 0 0 1 0 2 2 1"),
            2.2360679775
            )
    }

    #[test]
    fn jointjogjams_test3() {
        assert_eq!(
            test_fn(b"5 0 10 0 5 0 10 0"),
            5f64
            )
    }
}
