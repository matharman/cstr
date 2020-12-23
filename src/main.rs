use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use structopt::StructOpt;

enum Mode {
    Literal,
    #[allow(dead_code)]
    ConstChar(String),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "json2cstr")]
struct Opts {
    #[structopt(short, long)]
    stdin: bool,

    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn jsonstr_to_cstr(json: &str) -> String {
    json.replace("\"", "\\\"")
        .split("\n")
        .map(|x| format!("\"{}\"\n", x))
        .collect::<String>()
}

fn json_to_c(mode: Mode, json: &str) {
    match mode {
        Mode::ConstChar(var_name) => {
            println!(
                "const char {}[] = {};\n",
                var_name,
                jsonstr_to_cstr(json).trim()
            );
        }
        Mode::Literal => {
            println!("{}\n", jsonstr_to_cstr(json).trim());
        }
    }
}

#[allow(dead_code)]
fn file_to_var_name(path: &str) -> String {
    path.rsplit(|c| c == '/' || c == '\\')
        .next()
        .expect("Invalid filename")
        .replace("/", "_")
        .replace(" ", "_")
        .replace(".", "_")
}

fn consume_stdin() {
    let mut jsonvec: Vec<String> = Vec::new();
    io::stdin()
        .lock()
        .lines()
        .filter(|r| r.is_ok())
        .map(|ok| ok.unwrap())
        .for_each(|l| jsonvec.push(l.to_owned()));
    json_to_c(Mode::Literal, &jsonvec.join("\n").trim());
}

fn main() -> Result<(), ()> {
    let opts = Opts::from_args();

    if opts.stdin {
        consume_stdin();
    }

    opts.files.into_iter().for_each(|f| {
        let f = f.into_os_string().into_string().expect("Invalid path");
        let json = fs::read_to_string(&f).expect("Invalid file");
        json_to_c(Mode::Literal, json.trim());
    });

    Ok(())
}
