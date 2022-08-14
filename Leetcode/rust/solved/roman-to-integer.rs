use std::collections::HashMap;


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = [
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]
        .iter()
        .map(|x| *x)
        .collect::<HashMap<_, _>>();
        s.bytes().flat_map(|c| map.get(&c)).fold((0, 0), solve).0
    }
}

type State = (i32, i32);

fn solve((sum, carry): State, x: &i32) -> State {
    let stub1 = |c| {
        if carry == c {
            (sum + x - 2 * carry, 0)
        } else {
            (sum + x, 0)
        }
    };
    let stub2 = |c| {
        if carry == c {
            (sum + x - 2 * carry, 0)
        } else {
            (sum + x, *x)
        }
    };
    match x {
        5 => stub1(1),
        10 => stub2(1),
        50 => stub1(10),
        100 => stub2(10),
        500 | 1000 => stub1(100),
        _ => (sum + x, *x),
    }
}

