// Grouping values of different types
// You can have maximum 12 elements inside a tuple.

pub fn run() {
  let person: (&str, &str, i8) = ("Amruth", "Pillai", 24);

  println!("{} {} is {} years old.", person.0, person.1, person.2);
}
