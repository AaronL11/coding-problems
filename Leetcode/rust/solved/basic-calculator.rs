enum Expr {
    Var(i32),
    Neg(Option<Box<Expr>>),
    Plus(Option<Box<Expr>>, Option<Box<Expr>>),
}

fn parse(mut chars: impl Iterator<Item = char>) -> Option<Expr> {
    let mut stack = vec![];
    while let Some(c) = chars.next() {
        if c == ' ' {
            continue;
        }
        if c.is_ascii_digit() {
            if let Some(Some(expr)) = stack.pop() {
                let item = Expr::Var(c as i32 - 48);
                match expr {
                    Expr::Plus(None, None) => {
                        stack.push(Some(Expr::Plus(Some(Box::new(item)), None)))
                    }
                    Expr::Plus(x, None) => stack.push(Some(Expr::Plus(x, Some(Box::new(item))))),
                    Expr::Plus(x, Some(y)) => {
                        if let Expr::Neg(z) = *y {
                            if let None = z {
                                stack.push(Some(Expr::Plus(
                                    x,
                                    Some(Box::new(Expr::Neg(Some(Box::new(item))))),
                                )))
                            } else if let Expr::Var(n) = *z.unwrap() {
                                stack.push(Some(Expr::Plus(
                                    x,
                                    Some(Box::new(Expr::Neg(Some(Box::new(Expr::Var(
                                        n * 10 + (c as i32 - 48),
                                    )))))),
                                )))
                            }
                        } else if let Expr::Var(n) = *y {
                            stack.push(Some(Expr::Plus(
                                x,
                                Some(Box::new(Expr::Var(n * 10 + (c as i32 - 48)))),
                            )))
                        }
                    }
                    Expr::Neg(None) => stack.push(Some(Expr::Neg(Some(Box::new(item))))),
                    Expr::Neg(Some(x)) => {
                        if let Expr::Var(n) = *x {
                            stack.push(Some(Expr::Neg(Some(Box::new(Expr::Var(
                                n * 10 + (c as i32 - 48),
                            ))))))
                        }
                    }
                    Expr::Var(n) => stack.push(Some(Expr::Var(n * 10 + (c as i32 - 48)))),
                }
            } else {
                stack.push(Some(Expr::Var(c as i32 - 48)));
            }
        } else if c == ')' {
            if let Some(Some(expr1)) = stack.pop() {
                if let Some(Some(expr2)) = stack.pop() {
                    match expr2 {
                        Expr::Plus(Some(x), None) => {
                            stack.push(Some(Expr::Plus(Some(x), Some(Box::new(expr1)))));
                        }
                        Expr::Plus(Some(x), Some(y)) => {
                            if let Expr::Neg(None) = *y {
                                stack.push(Some(Expr::Plus(
                                    Some(x),
                                    Some(Box::new(Expr::Neg(Some(Box::new(expr1))))),
                                )))
                            }
                        }
                        Expr::Neg(None) => stack.push(Some(Expr::Neg(Some(Box::new(expr1))))),
                        _ => unreachable!(),
                    }
                } else {
                    match expr1 {
                        Expr::Var(x) => stack.push(Some(Expr::Var(x))),
                        Expr::Neg(Some(x)) => stack.push(Some(Expr::Neg(Some(x)))),
                        Expr::Plus(mut x, mut y) => {
                            stack.push(Some(Expr::Plus(x.take(), y.take())))
                        }
                        _ => unreachable!(),
                    }
                }
            }
        } else if c == '+' {
            if let Some(Some(expr)) = stack.pop() {
                stack.push(Some(Expr::Plus(Some(Box::new(expr)), None)))
            } else {
                stack.push(Some(Expr::Plus(None, None)))
            }
        } else if c == '-' {
            if let Some(Some(x)) = stack.pop() {
                stack.push(Some(Expr::Plus(
                    Some(Box::new(x)),
                    Some(Box::new(Expr::Neg(None))),
                )))
            } else {
                stack.push(Some(Expr::Neg(None)))
            }
        } else if c == '(' {
            stack.push(None);
        }
    }
    stack.pop().flatten()
}

fn eval(tree: Expr) -> i32 {
    match tree {
        Expr::Var(x) => x,
        Expr::Neg(Some(x)) => -eval(*x),
        Expr::Plus(Some(x), Some(y)) => eval(*x) + eval(*y),
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let iter = s.chars();
        if let Some(tree) = parse(iter) {
            eval(tree)
        } else {
            -1
        }
    }
}
