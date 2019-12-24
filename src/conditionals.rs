pub fn run() {
  let age: u8 = 24;
  let check_id: bool = true;
  let knows_person_of_age: bool = true;

  // If/Else If/Else
  if (age >= 21 && check_id) || knows_person_of_age {
    println!("Bartender: What's your poison?")
  } else if age < 21 && check_id {
    println!("Bartender: You're too young to drink!");
  } else {
    println!("Bartender: I'll need to see your ID...");
  }

  // Shorthand Conditional
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of age: {}", is_of_age);
}
