pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    //Basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Satvik", "NY");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
         "Satvik", "NY", "code"
    );

    //Named Arugments
    println!(
        "{name} likes to play {activity}",
         name = "John", 
         activity = "Baseball"
    );

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}