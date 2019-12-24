pub fn run() {
  let mut count: i8 = 0;

  // Infinite Loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }

  let mut fizz_count: i8 = 1;

  // While Loop (FizzBuzz)
  while fizz_count <= 100 {
    if fizz_count % 15 == 0 {
      println!("FizzBuzz");
    } else if fizz_count % 3 == 0 {
      println!("Fizz");
    } else if fizz_count % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", fizz_count);
    }

    // Increment Value
    fizz_count += 1;
  }

  // For Range Loop
  for x in 0..100 {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
  }
}
