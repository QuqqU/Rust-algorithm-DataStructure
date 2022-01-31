#[allow(dead_code)]
#[allow(unused, unused_mut)]
use scanner::Scanner;
use std::cmp;
use std::io::Read;

fn main() {
    let mut _s = String::new();
    std::io::stdin().read_to_string(&mut _s).unwrap();
    let mut sc = Scanner::new(&_s);

    ////



    ////
}

mod scanner {
    use std::str::FromStr;
    pub struct Scanner<'a> {
        it: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Self {
            Scanner {
                it: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            self.it.next().unwrap().parse().ok().unwrap()
        }
        pub fn next_vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
            (0..len).map(|_| self.next()).collect()
        }
        pub fn next_chars(&mut self) -> Vec<char> {
            self.it.next().unwrap().chars().collect()
        }
        pub fn ignore(&mut self, len: usize) {
            for _ in 0..len {
                self.it.next();
            }
        }
    }
}
