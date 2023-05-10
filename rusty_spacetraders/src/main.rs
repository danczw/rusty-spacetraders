fn register_agent(_callsign: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn main() {
    const CALLSIGN: &str = "snark";

    match register_agent(CALLSIGN) {
        Ok(_) => println!("Registered agent {}", CALLSIGN),
        Err(e) => println!("Error registering agent {}: {}", CALLSIGN, e),
    }
}
