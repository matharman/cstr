# cstr
A simple script consuming plaintext files, transforming them into C strings (by escaping `"` and `\` characters), and printing them to stdout for other tools to consume. Useful for generating C language test cases for string data.

## Building & installing
A convenience Makefile is provided, but typical `cargo build` and `cargo install` commands also work.
To install a debuggable version, run `make install PREFIX=/install/path`. To install a smaller version, run `make install-strip PREFIX=/install/path`.

## Usage
`cstr` has 2 usage modes: pass in a list of files, or pipe input to stdin.

For a list of files: `cstr /path/to/file1 /path/to/file2 ...`

For piping from stdin: `cat file1 | cstr --stdin`.

For example, this JSON file `fruit.json`:
````json
{
    "fruit": "Apple",
    "size": "Large",
    "color": "Red"
}
````

becomes
````C
"{"
"    \"fruit\": \"Apple\","
"    \"size\": \"Large\","
"    \"color\": \"Red\""
"}"
````
