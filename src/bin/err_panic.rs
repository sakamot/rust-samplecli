enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

use std::fmt;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

fn get_int_from_file() -> Result<i32, MyError>  {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;
    num_str // 最初は&str型
        .trim() // 文字列前後の空白文字を削除する。型は&str型
        .parse::<i32>() // &strをi32に変換する。結果は Result<i32, ParseIntError>型
        .map(|t| t * 2)  // parse()の結果がOkの時だけ実行する
        .map_err(|e| MyError::Num(e)) // parse()の結果がErr(e)の時だけ実行する
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e)
    }
}
