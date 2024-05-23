pub fn clear_console() {
    // Clear the screen and put the cursor at first row & first col of the screen
    print!("\x1B[2J\x1B[1;1H");
}

