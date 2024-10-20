use std::process::Command;
use std::fs;
use std::io::{self, Write};

fn deploy_pint_contract() {
    // Running the "pint build" command
    let output = Command::new("pint")
        .arg("build")
        .output()
        .expect("Failed to execute pint build");

    if output.status.success() {
        // If the command was successful
        println!("Contract built successfully.");
        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_contract_ids(&stdout);
    } else {
        // If the command failed
        eprintln!("Failed to build contract.");
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}

fn overwrite_last_n_chars(file_path: &str, overwrite_str: &str, n: usize) -> io::Result<()> {
    // Step 1: Read the entire content of the file
    let mut content = fs::read_to_string(file_path)?;

    // Step 2: Check if the content is long enough and overwrite the last n characters
    if content.len() >= n {
        // Truncate the content to remove the last n characters
        content.truncate(content.len() - n);
        // Append the overwrite string
        content.push_str(overwrite_str);
    } else {
        println!("The file has less than {} characters.", n);
        return Ok(()); // Exit early if the file is too short
    }

    // Step 3: Write the modified content back to the file
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn parse_contract_ids(output: &str) {
    // Split the output by lines
    let lines: Vec<&str> = output.lines().collect();

    for line in lines {
        // Find lines that start with "contract" or contain "::"
        if line.contains("::") || line.contains("      ") {
            // Split the line by whitespace and extract the last part (the contract ID)
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(contract_id) = parts.last() {
                println!("Found contract ID: {}", contract_id);
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Assume user input triggers the function
    let file_path = "/Users/jeffreyklinck/EthSF-1/project/order/src/unique.pnt"; // specify the target file
    let overwrite_str = "10;"; //Generate from a number

    let n = 3;

    overwrite_last_n_chars(file_path, overwrite_str, n)?;

    deploy_pint_contract();

    Ok(())
}


