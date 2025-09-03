use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("git/http-backend.c")?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"(?s)static struct service_cmd (?<content>.*)\};\r?\n\r?\n").unwrap();

    let Some(caps) = re.captures(&contents) else {
        println!("not found");
        return Ok(())
    };
    // Print the file contents
    //println!("File contents:\n{}", contents);

    println!("The service table is: {}", &caps["content"]);
    Ok(())
}