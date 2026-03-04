use std::fs;
use std::env;
use std::io::Write;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let _ = fs::create_dir_all("conf")?;
    if !Path::new("conf/.conf").exists() {
        let mut file = fs::File::create("conf/.conf")?;
        let _ = file.write_all(b"//Add paths to track");
        println!("Created conf/.conf\nAdd paths to the file to track them");
        assert!(env::set_current_dir("conf/").is_ok());
        return Ok(());
    }

    let content = fs::read_to_string("conf/.conf")?;
    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.starts_with("//") || line.is_empty(){
            continue;
        }
        let parts: Vec<&str> = line.split('/').collect();
        let name = parts.last().unwrap();
        let second_last = if parts.len() >= 2 { parts[parts.len() - 2].to_string() } else { format!("{}", i) };
        match fs::copy(line, format!("conf/{}_{}", second_last, name)) {
            Ok(_) => println!("Copied: {}", line),
            Err(e) => eprintln!("Failed to copy '{}': {}", line, e),
        }
    }
    
    assert!(env::set_current_dir("conf/").is_ok());

    Ok(())
}
