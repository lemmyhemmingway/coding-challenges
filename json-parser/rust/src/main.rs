use std::fs;

const INVALID_STEP1_PATH: &str = "../tests/step1/invalid.json";
fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Couldn't read the file {}")

}
fn parse_json(content: String) -> u8 {
    let trimmed_content = content.trim();
    if trimmed_content.eq("") {
        0
    } else {
        1
    }
}
fn main() {
    let output = read_file(INVALID_STEP1_PATH);
    let result = parse_json(output);
    println!("{}", result)
}

#[test]
fn test_empty_file() {
    let content = read_file(INVALID_STEP1_PATH);
    let result = parse_json(content);
    assert_eq!(0, result);

}
