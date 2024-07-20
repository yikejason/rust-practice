// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
pub(crate) fn two() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse().unwrap();
    let from_str: i32 = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}
