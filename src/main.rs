use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cstr")]
struct Opts {
    #[structopt(short, long)]
    stdin: bool,

    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn cstr_from_data(data: &str) -> String {
    data.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .split("\n")
        .map(|x| format!("\"{}\"\n", x))
        .collect::<String>()
}

fn consume_stdin() {
    let mut strvec: Vec<String> = Vec::new();
    io::stdin()
        .lock()
        .lines()
        .filter(|r| r.is_ok())
        .map(|ok| ok.unwrap())
        .for_each(|l| strvec.push(l.to_owned()));
    println!("{}", cstr_from_data(&strvec.join("\n").trim()));
}

fn main() -> Result<(), ()> {
    let opts = Opts::from_args();

    if opts.stdin {
        consume_stdin();
    }

    opts.files.into_iter().for_each(|f| {
        let f = f.into_os_string().into_string().expect("Invalid path");
        let data = fs::read_to_string(&f).expect("Invalid file");
        println!("{}", cstr_from_data(data.trim()));
    });

    Ok(())
}
