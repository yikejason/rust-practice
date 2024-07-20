use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
pub(crate) fn three() {
    // 使用两种方式填空
    // 不要修改其它地方的代码
    // let p: Result<Point, ParseIntError> = "(3, 4)".parse();
    let p = Point::from_str("(3, 4)");
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("Success!")
}
