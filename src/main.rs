use clap::Clap;

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
    formula_file: Option<String>, // 任意のオプション
}

fn main() {
    let opts = Opts::parse();

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}

/*
use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPM program")
        .version("1.0.0")
        .author("Your Name")
        .about("Super awesome sample RPM calculator")
        .arg(
            Arg::with_name("formula_file") // 数式を記載するファイルの指定
                .about("Formulas written in RPM")
                .value_name("FILE")
                .index(1) // 1番目の引数
                .required(false), // 必須項目ではない
        )
        .arg(
            Arg::with_name("verbose") // 出力を多くするためのオプション
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
*/
