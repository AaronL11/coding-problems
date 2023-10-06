#![allow(dead_code)]
#[allow(unused_imports)]
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    error, fmt,
    fmt::{Display, Formatter},
    io,
    io::{stdin, stdout, BufRead, BufWriter, Bytes, Read, Stdin, Stdout, Write},
    iter,
    iter::FromIterator,
    str,
    str::FromStr,
};

// Constants

type Int = isize;
type Uint = usize;
const EPSILON: f64 = 1e-5;
const INF: usize = 1_000_000_000;
const MOD: Uint = 1_000_000_007;
// const INF: Int = isize::MAX;

// Error handling

#[derive(Debug, Default)]
struct StopCode;

impl error::Error for StopCode {}

impl Display for StopCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Code failed to execute")
    }
}

impl From<io::Error> for StopCode {
    fn from(_: io::Error) -> StopCode {
        StopCode
    }
}

// Scanner code

struct Scanner<R>
where
    R: Read,
{
    bytes: Bytes<R>,
    buffer: Vec<u8>,
    split: str::SplitAsciiWhitespace<'static>,
}

#[allow(dead_code, unused_assignments)]
impl<R> Scanner<R>
where
    R: Read,
{
    fn new(bytes: Bytes<R>) -> Self {
        Self {
            bytes,
            buffer: Vec::new(),
            split: "".split_ascii_whitespace(),
        }
    }
    fn next<T: FromStr>(&mut self) -> Result<T, StopCode> {
        self.get_str()?.parse::<T>().ok().ok_or(StopCode)
    }
    fn get_str(&mut self) -> Result<&str, StopCode> {
        loop {
            if let Some(input) = self.split.next() {
                return Ok(input);
            }
            self.buffer.clear();
            loop {
                match self.bytes.next().ok_or(StopCode)? {
                    Ok(b'\n') => {
                        self.split = unsafe {
                            let slice = str::from_utf8_unchecked(&self.buffer);
                            std::mem::transmute(slice.split_ascii_whitespace())
                        };
                        break;
                    }
                    Ok(byte) => self.buffer.push(byte),
                    Err(_) => return Err(StopCode),
                }
            }
        }
    }
    fn flush(&mut self) {
        self.buffer.clear()
    }
    fn line(&mut self) -> Result<String, StopCode> {
        loop {
            match self.bytes.next().ok_or(StopCode)? {
                Ok(b'\n') => {
                    let result = unsafe { str::from_utf8_unchecked(&self.buffer).to_owned() };
                    self.buffer.clear();
                    return Ok(result);
                }
                Ok(byte) => self.buffer.push(byte),
                Err(_) => return Err(StopCode),
            }
        }
    }
    fn lines(&mut self) -> LineIter<R> {
        LineIter::new(self)
    }
    fn words<T: FromStr>(&mut self) -> WordsIter<T, R> {
        WordsIter::new(self)
    }
    fn take_into<T: FromStr, V: FromIterator<T>>(&mut self, n: usize) -> V {
        (0..n).flat_map(|_| self.next::<T>()).collect::<V>()
    }
    // Only works after const generics were stabilized!!
    fn take_tuple<T, V>(&mut self) -> Result<(T, V), StopCode>
    where
        T: FromStr,
        V: FromStr,
    {
        Ok((self.next::<T>()?, self.next::<V>()?))
    }
    fn take_tuple3<T, V, U>(&mut self) -> Result<(T, V, U), StopCode>
    where
        T: FromStr,
        V: FromStr,
        U: FromStr,
    {
        Ok((self.next::<T>()?, self.next::<V>()?, self.next::<U>()?))
    }
}

struct WordsIter<'a, T: FromStr, R: Read>(&'a mut Scanner<R>, std::marker::PhantomData<T>);

impl<'a, T: FromStr, R: Read> WordsIter<'a, T, R> {
    fn new(scan: &'a mut Scanner<R>) -> Self {
        Self(scan, std::marker::PhantomData)
    }
}

impl<'a, T, R> Iterator for WordsIter<'a, T, R>
where
    T: FromStr,
    R: Read,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next::<T>().ok()
    }
}

struct LineIter<'a, R: Read>(&'a mut Scanner<R>);

impl<'a, R: Read> LineIter<'a, R> {
    fn new(scan: &'a mut Scanner<R>) -> Self {
        Self(scan)
    }
}

// Solution Code Here

/// We start by creating a struct to represent our staff.
/// The first item is the survivor's current health.
/// The second is their starting health.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Survivor(u8, u8);

/// Then we define how we want to handle an attack event.
fn attack(
    zombies: &mut VecDeque<u8>,
    survivors: &mut BinaryHeap<Survivor>,
    ward: &mut BinaryHeap<Survivor>,
) -> Option<Option<Survivor>> {
    // Grab our zombie and staff member.
    // The question mark means it will fail early if the
    // staff queue is empty returning a `None`.
    let mut zombie = zombies.pop_front()?;
    let mut survivor = survivors.pop()?;
    // We need to compare the strength of the staff member
    // chosen relative to our zombie.
    // &zombie, &survivor;
    let result = match survivor.0.cmp(&zombie) {
        // If the zombie is too strong, then our survivor is totally damaged,
        // and we need a new survivor to finish the fight.
        // We first push the injured survivor on to the ward queue,
        // and then handle the zombie recursively.
        cmp::Ordering::Less => {
            zombie -= survivor.0;
            survivor.0 = survivor.1 / 2;
            ward.push(survivor);
            zombies.push_front(zombie);
            attack(zombies, survivors, ward)?
        }
        // If they are equal, then our survivor is totally damaged
        // but the zombie is dead.
        // We simply push them on to the ward queue.
        cmp::Ordering::Equal => {
            survivor.0 = survivor.1 / 2;
            ward.push(survivor);
            None
        }
        // If the survivor is strong enough, then we simply
        // remove the zombie's health from their health.
        // We then place them on the wait line.
        cmp::Ordering::Greater => {
            survivor.0 -= zombie;
            Some(survivor)
        }
    };
    // Possibly return the survivor who was used so they can be
    // reused later.
    Some(result)
}

#[allow(non_snake_case)]
fn solve(mut scan: Scanner<Stdin>, mut out: BufWriter<Stdout>) -> Result<(), StopCode> {
    // Gather the initial input and create a priority queue of our
    // survivors.
    let (S, m) = scan.take_tuple::<Uint, Uint>()?;
    let mut survivors = (0..S)
        .flat_map(|_| scan.next::<u8>())
        .map(|s| Survivor(s, s))
        .collect::<BinaryHeap<_>>();
    // This problem requires the use of a few data structures.
    // 1. A priority queue for our survivors, ordered by current health.
    // 2. A priority queue for our medical ward, ordered by starting health.
    // 3. A fifo queue for our incoming zombies.
    // 4. A fifo queue to store all of the survivors who just battled.
    let mut ward: BinaryHeap<Survivor> = BinaryHeap::with_capacity(S);
    let mut zombies = VecDeque::new();
    let mut wait = VecDeque::with_capacity(S);
    // We set a flag to change the output of the program.
    let mut good = true;
    // This just handles our wait stack for the case
    // where we only have one person added.
    let mut skip = false;
    // Get the number of events
    let e = scan.next::<Uint>()?;
    // Handle all our events line by line.
    for i in 1..=e {
        // Reset the medicine counter and grab any
        // available patients
        if i % m == 0 {
            if let Some(survivor) = ward.pop() {
                survivors.push(survivor)
            }
        }
        let string = scan.next::<String>()?;
        match &string[..] {
            "APPROACH" => zombies.push_back(scan.next::<u8>()?),
            "ATTACK" => {
                if let Some(res) = attack(&mut zombies, &mut survivors, &mut ward) {
                    // We attack our zombie and if there are any survivors
                    // get them to wait one whole event.
                    // If the wait queue is empty then the code below will try putting the
                    // survivor back in so we need to perform a skip.
                    if let Some(survivor) = res {
                        if wait.is_empty() {
                            skip = true;
                        }
                        wait.push_back(survivor);
                    }
                } else {
                    // Unsuccesful attack means we've been overrun, exit early!
                    good = false;
                    break;
                }
            }
            _ => unreachable!(),
        }
        // Put the next survivor back in.
        if !skip {
            if let Some(survivor) = wait.pop_front() {
                survivors.push(survivor);
            }
        } else {
            skip = false;
        }
    }
    // Check if we were successful.
    if good {
        writeln!(out, "success")?;
    } else {
        writeln!(out, "overrun")?;
    }
    Ok(out.flush()?)
}

#[allow(non_snake_case)]
fn main() -> Result<(), StopCode> {
    let scan = Scanner::new(stdin().bytes());
    let out = BufWriter::new(stdout());
    solve(scan, out)
}
