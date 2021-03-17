use std::process::Command;

fn main() {
    /*
    let child = Command::new("/bin/cat")
        .arg("colors")
        .current_dir("/home/james/Documents")
        .stdout(Stdio::piped())
        .spawn()
        .expect("command failed to start");
    */

    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .expect("can't run git!");

    println!("status: {}", output.status);
    println!("stdout: \n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: \n{}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}
