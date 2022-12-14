use std::str::FromStr;
use num::Complex;

fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(i) => {
            match  (T::from_str(&s[..i]), T::from_str(&s[i..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn test_pair() {
    assert_eq!(parse_pair::<i32>("1,", ','), None);
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex(""), None);
}
