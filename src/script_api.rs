use std::process::Command;

pub fn run_script(path: String) -> String {
    let script_output = Command::new("/bin/python3").arg(path).output().expect("failed to execute process");
    
    let script_stdout = String::from_utf8_lossy(&script_output.stdout).to_string();
    let script_stderr = String::from_utf8_lossy(&script_output.stderr);
    // let script_status = String::from_utf8_lossy(&script_output.status);
    
    //println!("status: {}", output.status);
    //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    script_stdout
}