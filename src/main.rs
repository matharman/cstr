use anyhow::{anyhow, Result};
use std::env;
use std::fs;

struct Opts {
    json_file: String,
    c_file: String,
    c_var: Option<String>,
}

impl Opts {
    fn parse(argv: &[String]) -> Result<Self> {
        // Usage
        // ./json2cstr <jsonfile> <cfile> [varname]
        match argv.len() {
            2 => Ok(Opts {
                json_file: argv[0].clone(),
                c_file: argv[1].clone(),
                c_var: None,
            }),
            3 => Ok(Opts {
                json_file: argv[0].clone(),
                c_file: argv[1].clone(),
                c_var: Some(argv[2].clone()),
            }),
            _ => Err(anyhow!(
                "Usage: json2cstr <input_json_file> <output_c_file> [c_var name]"
            )),
        }
    }

    fn json_to_c_str(json: &str) -> String {
        json.replace("\"", "\\\"")
            .split("\n")
            .map(|x| format!("\"{}\"\n", x))
            .collect::<String>()
    }

    fn emit_c_str(self) {
        let json = fs::read_to_string(self.json_file.clone()).unwrap();
        let c_var = self.c_var.unwrap_or(
            self.json_file
                .rsplit(|c| c == '/' || c == '\\')
                .next()
                .unwrap()
                .replace("/", "_")
                .replace(" ", "_")
                .replace(".", "_"),
        );
        let c_data = format!("const char {}[] = {};", c_var, Self::json_to_c_str(&json));

        fs::write(self.c_file.clone(), c_data).expect("Failed to write c file");
        println!("{}", self.c_file);
        println!("extern const char {}[];", c_var);
    }
}

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    Opts::parse(&argv[1..]).map(|opt| opt.emit_c_str())
}
