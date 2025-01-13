use psx::run;

fn normalize_string(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn it_converts_correctly() {
    use std::fs;

    let file_content = fs::read_to_string("tests/inputs/svg.1.svg").expect("Failed to read input file");
    let result = run(file_content, String::from("Svg"));

    let file_content_out = fs::read_to_string("tests/outputs/svg.1.jsx").expect("Failed to read expected output file");

    let normalized_result = normalize_string(&result);
    let normalized_expected = normalize_string(&file_content_out);

    if normalized_result != normalized_expected {
        println!("--- Actual Output ---\n{}", normalized_result);
        println!("--- Expected Output ---\n{}", normalized_expected);
        panic!("Outputs do not match. Please review the differences manually.");
    }
}

