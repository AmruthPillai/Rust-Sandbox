pub fn run() {
  // Print to Console
  println!("Hello from the print.rs file!");

  // Basic Formatting
  println!("{}'s last name is {}", "Amruth", "Pillai");

  // Positional Arguments
  println!(
    "{0}'s last name is {1} and {0} likes to {2}",
    "Amruth", "Pillai", "Code"
  );

  // Named Arguments
  println!(
    "{name} likes to play {game}",
    name = "Amruth",
    game = "Cricket"
  );

  // Placeholder Traits
  println!("Binary: {:b} Hexadecimal: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for Debug Trait
  println!("{:?}", (12, true, "Hello"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10);
}
