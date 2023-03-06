    pub fn next(msg:&str) {
        println!("\n\n\t\t\t{msg}");
        let mut response = String::new();
        std::io::stdin().read_line(&mut response).expect("Failed to read line");
    }

    pub fn get_input<T: std::str::FromStr>(question: &str) -> T {
        loop {
            use std::io::Write;
            print!("\n{question}"); // makes it look nicer
            std::io::stdout().flush().unwrap(); //prints the buffer
    
            let mut response = String::new();
            std::io::stdin().read_line(&mut response).expect("Failed to read line");
            match response.trim().parse() {
                Ok(val) => return val,
                Err(_) => {println!("\n\t\t\tError. Enter correct type.");continue;},
            }
        }
    }   

    pub fn menu(min:u8, max:u8, blacklisted: Vec<u8>) -> u8 {
        loop {
            let num:u8 = get_input("\nEnter option: ");
            if num >= min && num <= max && !(blacklisted.contains(&num)) {
                return num;
            }
            println!("\n\t\t\tPlease enter a valid option.");
        }
    } 