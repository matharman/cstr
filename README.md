# json2cstr
A simple tool consuming json files, transforming them into C strings, and printing them to stdout for other tools to consume. Useful for generating C language test cases for JSON parsers.

## Building & installing
A convenience Makefile is provided, but typical `cargo build` and `cargo install` commands also work.
To install a debuggable version, run `make install PREFIX=/install/path`. To install a smaller version, run `make install-strip PREFIX=/install/path`.

## Usage
`json2cstr` has 2 usage modes: pass in a list of JSON files, or pipe input to stdin.

For a list of JSON files: `json2cstr /path/to/file1.json /path/to/file2.json ...`

For piping from stdin: `cat file1.json | json2cstr --stdin var_name`.

For this sample `fruit.json`:
````json
{
    "fruit": "Apple",
    "size": "Large",
    "color": "Red"
}
````

The output would be
````C
const char var_name[] = "{"
"    \"fruit\": \"Apple\","
"    \"size\": \"Large\","
"    \"color\": \"Red\""
"}";

````
