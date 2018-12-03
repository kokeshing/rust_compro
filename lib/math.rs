// 累乗
fn power(n: i64, m: i64) -> i64 {
    match m {
        0 => 1,
        1 => n,
        _ => {
            let x = power(n, m / 2);
            if m % 2 == 0 {
                x * x
            } else {
                x * x * n
            }
        }
    }
}