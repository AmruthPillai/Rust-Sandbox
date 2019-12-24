// List of elements with same data type, usually of fixed length

use std::mem;

pub fn run() {
  let mut numbers: [i8; 5] = [1, 2, 3, 4, 5];

  // Reassign Values
  numbers[1] = 20;
  numbers[3] = 40;

  // Print Everything!
  println!("{:?}", numbers);

  // Get Single Value
  println!("numbers[0]: {}", numbers[0]);

  // Get Array Length
  println!("Length: {}", numbers.len());

  // Arrays are Stack Allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get a slice of the array
  let slice: &[i8] = &numbers[1..4];
  println!("{:?}", slice);
}
