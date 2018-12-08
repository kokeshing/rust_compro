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

fn mk_revel_barger(level : usize) -> Vec<usize> {
    if level == 1 {
        return vec!0, 1, 1, 1, 0];
    }

    let mut ans = vec![0];
    let mut level_1 = mk_revel_barger(level - 1);
    let mut level_1_cp: Vec<usize> = level_1.iter().map(|x| x.clone()).collect();

    ans.append(&mut level_1_cp);
    ans.append(&mut vec![1]);
    ans.append(&mut level_1);
    ans.append(&mut vec![0]);

    return ans;
}

fn main() {
    input!{
        n: usize,
        x: usize,
    }

    let mut barger = mk_revel_barger(n);
    println!("{}", mk_revel_barger(5));
    barger.reverse();
    let mut ans: u64 = 0;
    for i in 0..x {
        if barger[i] == 1 {
            ans += 1;
        }
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    writeln!(out, "{}", ans).unwrap();
}