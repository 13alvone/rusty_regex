extern crate argparse;
extern crate regex;

use {
    std::collections::BTreeMap,
    regex::Regex,
    std::str::FromStr,
    argparse::{ArgumentParser, Store},
};

fn make_replacements(_string: String) -> String {
    let references = BTreeMap::from([
        ("\\system32\\", "\\(s|S)ys(tem32|WOW64|wow64)\\"), // For Windows Paths
        ("\\System32\\", "\\(s|S)ys(tem32|WOW64|wow64)\\"), // For Windows Paths
        ("\\sysWOW64\\", "\\(s|S)ys(tem32|WOW64|wow64)\\"), // For Windows Paths
        ("\\SysWOW64\\", "\\(s|S)ys(tem32|WOW64|wow64)\\"), // For Windows Paths
    ]);

    let mut new_string = _string;
    for (bad_str, new_str) in references.iter() {
        new_string = fix_string(new_string, bad_str, new_str);
    }

    new_string
}

fn fix_string(_string: String, bad_str: &&str, new_str: &&str) -> String {
    let out_string = _string.replace(bad_str, new_str);
    out_string.to_string()
}

fn regex_guid(escape_str: String) -> String {
    // REGEX Cadence (GUID's)

    let regex_str = "([a-zA-Z0-9]{8}\\\\-([a-zA-Z0-9]{4}\\\\-){3}[a-zA-Z0-9]{12})";
    let reg_str = "([a-zA-Z0-9]{8}-([a-zA-Z0-9]{4}-){3}[a-zA-Z0-9]{12})";
    let re = Regex::from_str(regex_str).unwrap();
    let captures = re.captures(&escape_str);
    let mut guids = vec![];

    if captures.is_some() {
        for guid in captures.unwrap().iter() {
            let guid_str = guid.unwrap().as_str();
            if guid_str.len() == 40 && !guids.contains(&guid_str) {
                guids.push(guid_str);
            }
        }
    }

    // REPLACE All qualified guids with proper REGEX string
    let mut new_string = escape_str.to_string();
    for guid in guids {
        new_string = fix_string(new_string, &guid, &reg_str);
    }

    new_string

}

fn main() {

    // CMD Argument Cadence
    let mut input_string = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Adjust a given string in preparation for REGEX usage.");
        ap.refer(&mut input_string)
            .add_option(&["-s", "--input_string"], Store, "String to Regex.");
        ap.parse_args_or_exit();
    }

    // PROCESS Input and Produce Payload
    let escape_str = regex::escape(&input_string);
    let payload_string = make_replacements(regex_guid(escape_str));
    println!("[+] IN: \t{}\n[-] OUT: \t{}", &input_string, payload_string);

}
