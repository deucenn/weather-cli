use std::io::{self, Write};

fn get_city() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        println!("city?");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input);

        if input.is_empty() {
            return Ok(String::new());
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("city cannot be empty. try again");
            continue;
        }

        return Ok(trimmed.to_string());
    }
}

fn get_country() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        println!("country?");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input);

        if input.is_empty() {
            return Ok(String::new());
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("country cannot be empty. try again");
            continue;
        }

        return Ok(trimmed.to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut city_in = String::new();

    match get_city() {
        city => {
            city_in = city?;
        }
        Err(e) => {
            eprintln!("cant find city, error: {}", e);
        }
    }

    let mut country_in = String::new();

    match get_country() {
        country => {
            country_in = country?;
        }
        Err(e) => {
            eprintln!("cant find country, error: {}", e);
        }
    }

    println!("city: {}", city_in);
    println!("country: {}", country_in);

    Ok(())
}
