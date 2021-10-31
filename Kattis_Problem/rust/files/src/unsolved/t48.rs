use std::{io,error};
use std::io::BufRead;

#[allow(dead_code,unused_variables)]
fn main() -> Result<(),Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut board = [[0;4];4];
    for _ in 0..4 {
        stdin.read_line(&mut buf)?;
    }
    for (i,line) in buf.lines().enumerate() {
        for (j,piece) in line.split(' ').enumerate() {
            board[i][j] = piece.parse::<u16>()?;
        }
    }
    buf.clear();
    stdin.read_line(&mut buf)?;
    let n = buf.trim().parse::<u8>()?;
    
    match n {
        0 => left(&mut board),
        1 => up(&mut board),
        2 => right(&mut board),
        3 => down(&mut board),
        _ => panic!()
    }

    for x in board {
        println!("{} {} {} {}", x[0],x[1],x[2],x[3])
    }
    Ok(())
}

fn left(board: &mut [[u16;4];4]) {
    for i in 0..4 {
        for j in (0..4).rev() {
            if j!=4 && board[i][j]==board[i][j+1] {
                board[i][j+1] = board[i][j]*2;
                board[i][j] = 0;
            }
        }
    }
}

fn right(board: &mut [[u16;4];4]) {
    for i in 0..4 {
        for j in 0..4 {
            if j!=0 && board[i][j]==board[i][j+1] {
                board[i][j+1] = board[i][j]*2;
                board[i][j] = 0;
            }
        }
    }
}

fn up(board: &mut [[u16;4];4]) {
    for i in 0..4 {
        for j in (0..4).rev() {
            if j!=4 && board[i][j]==board[i+1][j] {
                board[i+1][j] = board[i][j]*2;
                board[i][j] = 0;
            }
        }
    }
}

fn down(board: &mut [[u16;4];4]) {
    for i in 0..4 {
        for j in 0..4 {
            if j!=0 && board[i][j]==board[i+1][j] {
                board[i+1][j] = board[i][j]*2;
                board[i][j] = 0;
            }
        }
    }
}


