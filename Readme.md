# rusty_regex
This project provides a binary that takes an input string, and preps it for regex usage, effectively replacing known generics and producing a usage-regex string for programmatic parsing.

## Build & MacOS Installation Instructions
> For Installing Rust and Cargo (Rust's Project Management Binary), please see: https://www.rust-lang.org/tools/install
> Then, run the following lines in terminal:
```sh
git clone https://git.corp.tanium.com/christopher-speakes/rusty_regex.git
cd rusty_regex
cargo build --bins --release
mv rusty_regex /usr/local/bin
```

## Usage and Flags
```sh
Usage:
  rusty_regex [OPTIONS]

Adjust a given string in preparation for REGEX usage.

Optional arguments:
  -h,--help             Show this help message and exit
  -s,--input_string INPUT_STRING
                        String to Regex.
```

## Example (Fake Data):
```sh
rusty_regex -s "C:\Windows\System32\WindowsPowerShell\{gf5f4454-3d4d-4ad4-9d90-57a4132a8a94}\(powershell).exe"
[+] IN:         C:\Windows\System32\WindowsPowerShell\{gf5f4454-3d4d-4ad4-9d90-57a4132a8a94}\(powershell).exe
[-] OUT:        C:\\Windows\\(s|S)ys(tem32|WOW64|wow64)\\WindowsPowerShell\\\{([a-zA-Z0-9]{8}-([a-zA-Z0-9]{4}-){3}[a-zA-Z0-9]{12})\}\\\(powershell\)\.exe

```
