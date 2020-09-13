use anyhow::{bail, ensure, Context, Result};

use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPM program",
    version = "1.0.0",
    author = "Your Name",
    about = "Super awesome sample PRM calculator"
)]

struct Opts {
    // / Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // / Formulas written in RPM
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>, // 任意のオプション
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        // スタック
        // pop()は末尾から行われるのでrev()をする
        // collect(): イテレータをコレクションに変換するめそっど
        // Vec<_>:
        // Vecの要素型をわざわざ指定しなくても_で埋めておくことでコンパイラが適切な型を決めてくれる
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_liner(&mut tokens)
    }

    pub fn eval_liner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            // -vオプションが指定されている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}

// アトリビュート
// cfg: コンパイル時の設定によって有効無効を切り替えられる
// ↓の場合はbuildやrunでは無効になり、testの時だけ有効になる
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}
