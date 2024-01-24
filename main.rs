use std::fs::File;
use std::io::{self, Write};

fn generate_permutations(chars: &str, length: usize, file: &mut File) -> io::Result<()> {
    // Iterate through all permutations
    for (index, permuted_index) in (0..chars.len().pow(length as u32)).enumerate() {
        let formatted_str: String = (0..length)
            .rev()
            .map(|i| chars.chars().nth((permuted_index / chars.len().pow(i as u32)) % chars.len()).unwrap())
            .collect();

        // Write the formatted string to the file
        writeln!(file, "{}", formatted_str)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Open a file for writing
    let mut file = File::create("output.txt")?;

    let characters = "abcdefghijklmnopqrstuvwxyz_";
    let length = 4;

    generate_permutations(characters, length, &mut file)?;

    println!("Program executed successfully. Check 'output.txt' for the result.");

    Ok(())
}
