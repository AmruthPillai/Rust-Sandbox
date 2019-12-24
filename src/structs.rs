// Structs are used to create custom data types

// Primitive Struct
struct PrimitiveColor {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Complex Struct
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Creating New Person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  // Get Full Name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Change Last Name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name -> Tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut prim_color = PrimitiveColor {
    red: 255,
    green: 0,
    blue: 0,
  };
  prim_color.red = 200;
  println!(
    "Primitive Color: {} {} {}",
    prim_color.red, prim_color.green, prim_color.blue
  );

  let mut tupl_color = TupleColor(255, 0, 0);
  tupl_color.1 = 200;
  println!(
    "Tuple Color: {} {} {}",
    tupl_color.0, tupl_color.1, tupl_color.2
  );

  let person1: Person = Person::new("John", "Doe");
  println!("Full Name of Person: {}", person1.full_name());

  let mut person2: Person = Person::new("Mary", "Doe");
  person2.set_last_name("Williams");
  println!("Full Name of Person: {}", person2.full_name());
  println!("Person Tuple: {:?}", person2.to_tuple());
}
