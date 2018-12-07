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

fn calc_temp(t:f32, x: f32) -> f32 {
    return t - x * 0.006;
}

fn main() {
    input!{
        n: usize,
        t: isize,
        a: isize,
        hs: [isize; n],
    }
    let mut min = 9999.0;
    let mut ans = 0;

    for i in 0..n {
        let tmp = (calc_temp(t as f32, hs[i] as f32) - a as f32).abs();
        if tmp < min {
            min = tmp;
            ans = i + 1;
        }
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", ans).unwrap();
}