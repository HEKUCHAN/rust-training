#![allow(non_snake_case)]
struct FibIterator {
    a: usize,
    b: usize,
}

impl FibIterator {
    fn new() -> Self { FibIterator {a: 1, b: 1} }
}

impl Iterator for FibIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        return Some(self.a)
    }
}

fn main() {
    let fib_iter = FibIterator::new();
    for (i, n) in fib_iter.enumerate() {
        if i >= 10 { break; }
        print!("{},", n);
    }
    println!("");
}
