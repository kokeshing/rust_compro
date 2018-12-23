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
// a = 2, 7, 61                   u32程度まで
// a = 2, 3, 5, 7, 11, 13, 17     30^14程度まで
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

// 試し割り法 素因数分解 [2, 2, 2, 2, 2, 3, 3, 5 ...]の形式で帰ってくる
fn trivial_division(n: usize) -> Vec<usize> {
    let mut result = n;
    for i in 2..((n as f64).sqrt() as usize)+1 {
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
