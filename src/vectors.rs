// Vectors are Resizable Arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Reassign Values
  numbers[1] = 20;
  numbers[3] = 40;

  // Add Items to the Vector
  numbers.push(6);
  numbers.push(7);

  // Remove Last Item from Vector
  numbers.pop();

  // Print Everything!
  println!("{:?}", numbers);

  // Get Single Value
  println!("numbers[0]: {}", numbers[0]);

  // Get Vector Length
  println!("Length: {}", numbers.len());

  // Vectors are Stack Allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get a slice of the Vector
  let slice: &[i32] = &numbers[1..4];
  println!("{:?}", slice);

  // Loop through Vectors
  for x in slice.iter() {
    println!("Number: {}", x);
  }

  // Loop and Mutate
  for x in numbers.iter_mut() {
    *x *= 2; // Multiply each item by two
  }

  println!("{:?}", numbers);
}
