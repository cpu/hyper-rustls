use std::{env, io};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time;

fn examples_dir() -> PathBuf {
    let target_dir: PathBuf = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string())
        .into();
    target_dir
        .join("debug")
        .join("examples")
}

fn server_command() -> Command {
    Command::new(examples_dir().join("server"))
}

fn client_command() -> Command {
    Command::new(examples_dir().join("client"))
}

#[test]
fn client() {
    let rc = client_command()
        .arg("https://google.com")
        .output()
        .expect("cannot run client example");

    assert!(rc.status.success());
}

#[test]
fn server() {
    let mut srv = server_command()
        .arg("1337")
        .spawn()
        .expect("cannot run server example");

    thread::sleep(time::Duration::from_secs(1));

    let output = Command::new("which")
        .arg("curl")
        .output()
        .expect("couldn't which curl");
    let str_output = String::from_utf8_lossy(&*output.stdout);
    println!("curl bin: {:?}", str_output);

    let output = Command::new("curl")
        .arg("--version")
        .output()
        .expect("failed to get curl version");
    let str_output = String::from_utf8_lossy(&*output.stdout);
    println!("curl version: {:?}", str_output);

    let output = Command::new("curl")
        .arg("--insecure")
        .arg("--http1.0")
        .arg("--verbose")
        .arg("https://localhost:1337")
        .output()
        .expect("cannot run curl");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let str_output = String::from_utf8_lossy(&*output.stdout);
    println!("client output: {:?}", str_output);
    assert_eq!(output.stdout, b"Try POST /echo\n");

    srv.kill().unwrap();
}

#[test]
fn custom_ca_store() {
    let mut srv = server_command()
        .arg("1338")
        .spawn()
        .expect("cannot run server example");

    thread::sleep(time::Duration::from_secs(1));

    let rc = client_command()
        .arg("https://localhost:1338")
        .arg("examples/sample.pem")
        .output()
        .expect("cannot run client example");

    srv.kill().unwrap();

    if !rc.status.success() {
        assert_eq!(String::from_utf8_lossy(&rc.stdout), "");
    }
}
