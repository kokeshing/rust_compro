use std::io::{stdout, Write, BufWriter};
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };

    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn trivial_division(n: u64) -> Vec<u64> {
    let mut result = n;
    for i in 2..((n as f64).sqrt() as u64)+1 {
        if n % i == 0 {
            result = i;
            break;
        }
    }

    if result == n {
        return vec![n];
    } else {
        let mut v1 = vec![result];
        let mut v2 = trivial_division(n / result);

        v1.append(&mut v2);

        return v1;
    }
}

use std::collections::HashSet;
fn main() {
    input!{
        n: usize,
        p: usize,
    }

    let sosus = trivial_division(p as u64);
    let unique: HashSet<u64> = sosus.clone().into_iter().collect();

    let mut ans: u64 = 1;
    for sosu in unique {
        let kosu: u64 = (sosus.iter().filter(|&&e| e == sosu).count() / n) as u64;
        if kosu > 0 {
            ans = ans * sosu.pow(kosu as u32);
        }
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    writeln!(out, "{}", ans).unwrap();
}