use float_next_after::NextAfter;

static PI: f64 = 3.141592653589793;

pub fn fract(n: f64) -> f64 {
    n - n.floor()
}

pub fn pseudohash(s: &String) -> f64 {
    let mut num = 1.0;
    for i in (1..=s.len()).rev() {
        let c = s.chars().nth(i - 1).unwrap();
        num = fract(1.1239285023 / num * (c as u8 as f64) * PI + PI * i as f64);
    }
    if num.is_nan() {
        return f64::NAN;
    }
    num
}

pub fn round13(x: f64) -> f64 {
    let inv_prec: f64 = 10f64.powi(13);
    let two_inv_prec: f64 = 2f64.powi(13);
    let five_inv_prec: f64 = 5f64.powi(13);

    let tentative = (x * inv_prec).floor() / inv_prec;
    let truncated = ((x * two_inv_prec) % 1.0) * five_inv_prec;
    if tentative != x && truncated % 1.0 >= 0.5 && tentative != x.next_after(1.0) {
        return ((x * inv_prec).floor() + 1.0) / inv_prec;
    }
    tentative
}