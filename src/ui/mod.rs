
use std::io::{self, Write};



mod print_pos;

#[inline]
fn clear_screen(writer: &mut io::Stdout) {
    // Clear the terminal.
    writer.write_all(b"\x1b[2J").unwrap();

    // Move the cursor to the top left corner of the terminal.
    writer.write_all(b"\x1b[H").unwrap();
}

pub fn term_main() {
    // Create a new terminal writer.
    let mut writer: io::Stdout = io::stdout();

    loop {
        clear_screen(&mut writer);
        writer.write_all(b"Enter a string: ").unwrap();

        // Read a string from the terminal.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Print the string that was read from the terminal.
        println!("You entered: {}", input);

        // Check if the user entered a string that is not empty.
        if input.len() > 0 {
            break;
        }

        // Write a message to the terminal if the user did not enter a string that is not empty.
        writer.write_all(b"Please enter a string: ").unwrap();
    }
}
