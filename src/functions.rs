pub fn run() {
  greeting("Hello", "Amruth");

  // Bind Function Return Value to Variables
  let sum = add(5, 5);
  println!("Sum: {}", sum);

  // Closure
  let z: i32 = 10;
  let add_num = |x: i32, y: i32| x + y;
  println!("Closure Sum: {}", add_num(5, z));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}
