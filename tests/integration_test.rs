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
fn test__() {
    let args = []; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
   assert_eq!(ls_output, rust_ls_output, "Outputs of ls and rust_ls do not match");
}

#[test]
fn test_l() {
    let args = ["-l"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -l and rust_ls -l do not match");
}

#[test]
fn test_f() {
    let args = ["-f"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -f and rust_ls -f do not match");
}

#[test]
fn test_C() {
    let args = ["-C"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -C and rust_ls -C do not match");
}
#[test]
fn test_x() {
    let args = ["-x"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -x and rust_ls -x do not match");
}

#[test]
fn test_xi() {
    let args = ["-xi"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -xi and rust_ls -xi do not match");
}
#[test]
fn test_g() {
    let args = ["-g"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -g and rust_ls -g do not match");
}

#[test]
fn test_o() {
    let args = ["-o"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -o and rust_ls -o do not match");
}

#[test]
fn test_Ci() {
    let args = ["-Ci"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -Ci and rust_ls -Ci do not match");
}

#[test]
fn test_1() {
    let args = ["-1"]; // Long listing f ormat argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -1 and rust_ls -1 do not match");
}

#[test]
fn test_i() {
    let args = ["-i"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -i and rust_ls -i do not match");
}

#[test]
fn test_a() {
    let args = ["-a"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -a and rust_ls -a do not match");
}

#[test]
fn test_A() {
    let args = ["-A"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -A and rust_ls -A do not match");
}

#[test]
fn test_m() {
    let args = ["-m"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -m and rust_ls -m do not match");
}


#[test]
fn test_r() {
    let args = ["-r"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -r and rust_ls -r do not match");
}
#[test]
fn test_lr() {
    let args = ["-lr"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -r and rust_ls -r do not match");
}


#[test]
fn test_q() {
    let args = ["-q"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -q and rust_ls -q do not match");
}


#[test]
fn test_n() {
    let args = ["-n"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -n and rust_ls -n do not match");
}

#[test]
fn test_F() {
    let args = ["-F"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -F and rust_ls -F do not match");
}

#[test]
fn test_s() {
    let args = ["-s"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -s and rust_ls -s do not match");
}


#[test]
fn test_d() {
    let args = ["-d"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -d and rust_ls -d do not match");
}


#[test]
fn test_S() {
    let args = ["-S"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
    assert_eq!(ls_output, rust_ls_output, "Outputs of ls -S and rust_ls -S do not match");
}
#[test]
fn test_p() {
    let args = ["-p"]; // Long listing format argument
    let ls_output = run_command("ls", &args);
    let rust_ls_output = run_command("./target/debug/rust_ls", &args);
   assert_eq!(ls_output, rust_ls_output, "Outputs of ls -p and rust_ls -p do not match");
}