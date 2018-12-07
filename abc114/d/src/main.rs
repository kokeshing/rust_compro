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

fn main() {
    input!{
        n: usize,
    }

    let mut pf_map = vec![1; n-1];
    for num in (2..n+1) {
        let mut tmp = num;
        for (i, x) in (2..n+1).enumerate() {
            while tmp % x == 0 {
                pf_map[i] += 1;
                tmp = tmp / x;
            }
        }
    }

    let ans = &pf_map.iter().filter(|x| **x >= 75).count()
            + &pf_map.iter().filter(|x| **x >= 25).count() * (&pf_map.iter().filter(|x| **x >= 3).count() - 1)
            + &pf_map.iter().filter(|x| **x >= 15).count() * (&pf_map.iter().filter(|x| **x >= 5).count() - 1)
            + &pf_map.iter().filter(|x| **x >= 5).count() * (&pf_map.iter().filter(|x| **x >= 5).count() - 1) * (&pf_map.iter().filter(|x| **x >= 3).count() - 2) / 2;

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", ans).unwrap();
}