use std::process::Command;

fn run_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute command");

    assert!(output.status.success(), "Command did not run successfully");

    // Trim and return the output as a string
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn test_rust_ls_vs_ls_long_format() {
    let args = ["-l"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -l and rust_ls -l do not match");
}

#[test]
fn test_rust_ls_vs_ls_i() {
    let args = ["-i"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -i and rust_ls -i do not match");
}

#[test]
fn test_rust_ls_vs_ls_m() {
    let args = ["-m"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -m and rust_ls -m do not match");
}


#[test]
fn test_rust_ls_vs_ls_n() {
    let args = ["-n"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -n and rust_ls -n do not match");
}


#[test]
fn test_rust_ls_vs_ls_S() {
    let args = ["-S"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -S and rust_ls -S do not match");
}

#[test]
fn test_rust_ls_vs_ls() {
    let args = []; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
   assert_eq!(ls_output, rust_ls_output, "Outputs of ls and rust_ls do not match");
}