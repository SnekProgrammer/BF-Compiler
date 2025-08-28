use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

/// Runs the compiler and executable for each test case in tests/*/ directories.
#[test]
fn test_bf_compiler_outputs() {
    let target_arch = if cfg!(target_os = "windows") { "win64" } else { "unix" };
    let test_out_dir = Path::new("test_out");
    // Ensure test_out directory exists
    if !test_out_dir.exists() {
        fs::create_dir(test_out_dir).expect("Failed to create test_out directory");
    }
    // Clean up before
    if test_out_dir.exists() {
        for entry in fs::read_dir(test_out_dir).expect("Failed to read test_out directory") {
            let entry = entry.expect("Failed to get entry");
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path).expect("Failed to remove test_out file");
            }
        }
    }
    let test_dir = Path::new("tests");
    for entry in fs::read_dir(test_dir).expect("Failed to read tests directory") {
        let entry = entry.expect("Failed to get entry");
        let case_dir = entry.path();
        if case_dir.is_dir() {
            let bf_file = case_dir.join("prog.b");
            let in_file = case_dir.join("input.in");
            let out_file = case_dir.join("expected.out");
            let file_stem = case_dir.file_name().unwrap().to_string_lossy();
            let asm_file = format!("test_out/{}.temp.asm", file_stem);
            let obj_file = format!("test_out/{}.temp.o", file_stem);
            let exe_file = format!("test_out/{}", file_stem);
            // Compile
            let status = Command::new("cargo")
                .args([
                    "run", "--release", "--",
                    bf_file.to_str().unwrap(),
                    "-o", &format!("test_out/{}", file_stem),
                    "-p", target_arch,
                    "-a"
                ])
                .status()
                .expect("Failed to run compiler");
            assert!(status.success(), "Compiler failed for {}", bf_file.display());
            println!("{}", Path::new(&asm_file).to_str().unwrap_or("Failed to unwrap path"));
            println!("{}", Path::new(&obj_file).to_str().unwrap_or("Failed to unwrap path"));
            println!("{}", Path::new(&exe_file).to_str().unwrap_or("Failed to unwrap path"));
            assert!(Path::new(&asm_file).exists(), "Missing asm file for {}", bf_file.display());
            assert!(Path::new(&obj_file).exists(), "Missing obj file for {}", bf_file.display());
            assert!(Path::new(&exe_file).exists(), "Missing exe file for {}", bf_file.display());
            // Run executable if input/output files exist
            if in_file.exists() && out_file.exists() {
                let input = fs::read(&in_file).expect("Failed to read input file");
                let expected = fs::read(&out_file).expect("Failed to read expected output file");
                let output = Command::new(exe_file.clone())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        use std::io::Write;
                        child.stdin.as_mut().unwrap().write_all(&input)?;
                        let output = child.wait_with_output()?;
                        Ok(output.stdout)
                    })
                    .expect("Failed to run executable");
                assert!(output == expected, "Output mismatch for {}\nExpected (bytes): {:?}\nActual (bytes): {:?}\nExpected (chars): {}\nActual (chars): {}", bf_file.display(), expected, output, String::from_utf8_lossy(&expected), String::from_utf8_lossy(&output));
            }
        }
    }
    // Clean up after
    if test_out_dir.exists() {
        for entry in fs::read_dir(test_out_dir).expect("Failed to read test_out directory") {
            let entry = entry.expect("Failed to get entry");
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path).expect("Failed to remove test_out file");
            }
        }
    }
}
