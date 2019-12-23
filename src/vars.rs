pub fn run() {
  let name = "Amruth Pillai";
  let mut age = 24;
  println!("My name is {} and I am {} years old.", name, age);
  age = 25;
  println!("My name is {} and I am {} years old.", name, age);

  // Defining Constants
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign Multiple at Once
  let (my_name, my_age) = ("Amruth Pillai", 24);
  println!("{} is {}", my_name, my_age);
}
