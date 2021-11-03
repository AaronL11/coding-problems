use std::io;
use io::{Read,Write};

fn main() -> io::Result<()> {
    let mut buf: [u8;15_000] = [0;15_000];
    let n = io::stdin().lock().read(&mut buf)?-1;
    dbg!(&n);
    let (l,r) = buf[..n].split_at_mut(n/2);
    for i in 0..n/2 {
        l[i] -= 65;
        r[i] -= 65;
    }
    let (sl,sr) = (l.iter().sum::<u8>(),
                    r.iter().sum::<u8>());
    for i in 0..n/2 {
        l[i] = (((l[i]+sl)%26+(r[i]+sr)%26)%26)+65
    }
    io::stdout().lock().write_all(&l)
}
