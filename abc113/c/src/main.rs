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
        m: usize,
        p: [(usize, usize); m],
    }
    let mut py: Vec<((usize, usize), usize)> = p.into_iter().zip(1..).collect();
    py.sort_by_key(|t| (t.0).1);

    let mut kens = vec![1; n];
    for i in 0..m {
        (py[i].0).1 = kens[(py[i].0).0 - 1];
        kens[(py[i].0).0 - 1] += 1;
    }
    py.sort_by_key(|t| t.1);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for i in 0..m {
        writeln!(out, "{0: >06}{1: >06}", (py[i].0).0, (py[i].0).1).unwrap();
    }
}