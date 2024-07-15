// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     // 实现 from 方法
//     fn from(err: io::Error) -> Self {
//         CliError::IoError(err)
//     }
// }

// impl From<num::ParseIntError> for CliError {
//     // 实现 from 方法
//     fn from(err: num::ParseIntError) -> Self {
//         CliError::ParseError(err)
//     }
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? 自动将 io::Error 转换成 CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

pub(crate) fn three() {
    println!("Success!")
}
