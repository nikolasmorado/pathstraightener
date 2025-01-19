use psx::run;

fn normalize_string(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn svg_to_jsx() {
    use std::fs;

    let file_content = fs::read_to_string("tests/inputs/svg.1.svg").expect("Failed to read input file");
    let result = run(file_content, String::from("Psx"), false, false);

    let file_content_out = fs::read_to_string("tests/outputs/svg.1.jsx").expect("Failed to read expected output file");

    let normalized_result = normalize_string(&result);
    let normalized_expected = normalize_string(&file_content_out);

    if normalized_result != normalized_expected {
        println!("--- Actual Output ---\n{}", normalized_result);
        println!("--- Expected Output ---\n{}", normalized_expected);
        panic!("Outputs do not match. Please review the differences manually.");
    }
}

#[test]
fn svg_to_tsx() {
    use std::fs;

    let file_content = fs::read_to_string("tests/inputs/svg.1.svg").expect("Failed to read input file");
    let result = run(file_content, String::from("Psx"), true, false);

    let file_content_out = fs::read_to_string("tests/outputs/svg.1.tsx").expect("Failed to read expected output file");

    let normalized_result = normalize_string(&result);
    let normalized_expected = normalize_string(&file_content_out);

    if normalized_result != normalized_expected {
        println!("--- Actual Output ---\n{}", normalized_result);
        println!("--- Expected Output ---\n{}", normalized_expected);
        panic!("Outputs do not match. Please review the differences manually.");
    }
}

#[test]
fn svg_to_jsx_rn() {
    use std::fs;

    let file_content = fs::read_to_string("tests/inputs/svg.1.svg").expect("Failed to read input file");
    let result = run(file_content, String::from("Psx"), false, true);

    let file_content_out = fs::read_to_string("tests/outputs/svg_rn.1.jsx").expect("Failed to read expected output file");

    let normalized_result = normalize_string(&result);
    let normalized_expected = normalize_string(&file_content_out);

    if normalized_result != normalized_expected {
        println!("--- Actual Output ---\n{}", normalized_result);
        println!("--- Expected Output ---\n{}", normalized_expected);
        panic!("Outputs do not match. Please review the differences manually.");
    }
}

#[test]
fn svg_to_tsx_rn() {
    use std::fs;

    let file_content = fs::read_to_string("tests/inputs/svg.1.svg").expect("Failed to read input file");
    let result = run(file_content, String::from("Psx"), true, true);

    let file_content_out = fs::read_to_string("tests/outputs/svg_rn.1.tsx").expect("Failed to read expected output file");

    let normalized_result = normalize_string(&result);
    let normalized_expected = normalize_string(&file_content_out);

    if normalized_result != normalized_expected {
        println!("--- Actual Output ---\n{}", normalized_result);
        println!("--- Expected Output ---\n{}", normalized_expected);
        panic!("Outputs do not match. Please review the differences manually.");
    }
}

