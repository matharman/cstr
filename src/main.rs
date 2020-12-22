#[macro_use]
extern crate anyhow;

use anyhow::Result;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "json2cstr")]
struct Opts {
    #[structopt(short, long)]
    stdin: bool,

    #[structopt(name = "C variable name", default_value = "blob")]
    var_name: String,

    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn jsonstr_to_cstr(json: &str) -> String {
    json.replace("\"", "\\\"")
        .split("\n")
        .map(|x| format!("\"{}\"\n", x))
        .collect::<String>()
}

fn json_to_c(json: &str, var_name: &str) {
    println!(
        "const char {}[] = {};\n",
        var_name,
        jsonstr_to_cstr(json).trim()
    );
}

fn file_to_var_name(path: &str) -> String {
    path.rsplit(|c| c == '/' || c == '\\')
        .next()
        .expect("Invalid filename")
        .replace("/", "_")
        .replace(" ", "_")
        .replace(".", "_")
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    if opts.stdin {
        let mut jsonvec: Vec<String> = Vec::new();
        io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap())
            .for_each(|l| jsonvec.push(l.to_owned()));
        json_to_c(&jsonvec.join("\n"), &opts.var_name);
    } else {
        opts.files.into_iter().for_each(|f| {
            let f = f.into_os_string().into_string().expect("Invalid path");
            let json = fs::read_to_string(&f).expect("Invalid file");
            json_to_c(json.trim(), &file_to_var_name(&f));
        });
    }

    Ok(())
}
