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

// バイナリ法 冪剰余の計算 b^e mod m
fn binaly_method(mut b: usize, e: usize, m: usize) -> usize {
    let mut ret = 1;
    let mut i = 1;

    while i < e {
        if (e & i) != 0 {
            ret = (ret * b) % m;
        }
        b = (b * b) % m;
        i <<= 1;
    }

    return ret;
}

// ミラーラビン素数判定法
// a = 2, 7, 61 u32程度まで
fn miller_rabin_primality_test(n: usize) -> bool {
    if n == 2 {
        return true;
    }

    if n == 1 || n & 1 == 0 {
        return false;
    }

    let mut d = n - 1;
    while d & 1 == 0 {
        d =  d >> 1;
    }

    let nums = [2, 7, 61];
    for &a in nums.into_iter() {
        if a >= n {
            break;
        }
        let mut t = d;
        let mut y = binaly_method(a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y * y) % n;
            t = t << 1;
        }

        if y != n - 1 && t & 1 == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    assert_eq!(miller_rabin_primality_test(5), true);
    assert_eq!(miller_rabin_primality_test(7), true);
    assert_eq!(miller_rabin_primality_test(11), true);
    assert_eq!(miller_rabin_primality_test(13), true);
    assert_eq!(miller_rabin_primality_test(17), true);
    assert_eq!(miller_rabin_primality_test(23), true);
    assert_eq!(miller_rabin_primality_test(29), true);
    assert_eq!(miller_rabin_primality_test(31), true);
    assert_eq!(miller_rabin_primality_test(37), true);
    assert_eq!(miller_rabin_primality_test(43), true);
    assert_eq!(miller_rabin_primality_test(307), true);
    assert_eq!(miller_rabin_primality_test(421), true);
    assert_eq!(miller_rabin_primality_test(2003), true);
    assert_eq!(miller_rabin_primality_test(2207), true);
    assert_eq!(miller_rabin_primality_test(3571), true);
}