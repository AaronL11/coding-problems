mod tools;
use tools::*;

fn median(l: &mut [i32]) -> i32 {
    l.sort();
    let n = l.len();
    if n%2==0 {
        (l[n/2]+l[n/2-1])/2
    } else {
        l[n/2]
    }
}

#[allow(unused_variables, non_snake_case)]
fn main() -> Result<(), Box<dyn error::Error>> {
    let T = input::<i32>()?;
    for _ in 0..T {
        let N = input::<i32>()?;
        let mut A = vec_from_input::<i32>(" ")?;
        let mut result = 0;
        for i in 0..A.len() {
            let l = &mut A[..i+1];
            result += median(&mut l[..]);         
        }
        print!("{}\n",result)
    }
    Ok(())
}