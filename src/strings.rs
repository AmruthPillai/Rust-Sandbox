pub fn run() {
  // Primitive String
  let hello = "Hello";

  // Heap Allocated String
  let mut world = String::from("World");

  // Push Character
  world.push('!');

  // Push String
  world.push_str(" Welcome!");

  // Capacity in Bytes
  println!("Capacity: {}", world.capacity());

  // Check if Empty
  println!("isEmpty: {}", world.is_empty());

  // Check if Contains
  println!("Contains 'World': {}", world.contains("World"));

  // Replace
  println!(
    "Replace 'Welcome' to 'Namaste': {}",
    world.replace("Welcome", "Namaste")
  );

  // Print Everything
  println!("{}, {}", hello, world);

  // Get Length of String
  println!("Hello: {}", hello.len());
  println!("World: {}", world.len());

  // Loop Through String by Whitespace
  for word in world.split_whitespace() {
    println!("{}", word);
  }

  // Create String with Limited Capacity
  let mut s = String::with_capacity(5);
  s.push('a');
  s.push('b');
  s.push('c');
  s.push('d');
  s.push('e');

  println!("{}", s);

  // Assertion Testing
  assert_eq!(5, s.len());
  assert_eq!(5, s.capacity());
}
