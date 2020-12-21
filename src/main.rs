#[macro_use]
extern crate anyhow;

use anyhow::Result;
use std::env;
use std::fs;

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
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        return Err(anyhow!("Usage: json2cstr [JSON FILES]"));
    }

    argv[1..].into_iter().for_each(|f| {
        let json = fs::read_to_string(&f).expect("Invalid file");
        json_to_c(json.trim(), &file_to_var_name(&f));
    });

    Ok(())
}
