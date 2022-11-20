use std::str::FromStr;

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

fn main() {
    println!("Hello, world!");
}


#[test]
fn test_pair() {
    assert_eq!(parse_pair::<i32>("1,", ','), None);
}
