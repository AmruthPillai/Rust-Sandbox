// Pointer Reference: Points to a resouce in memory

pub fn run() {
  // Arrays
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2));

  // Vectors
  let vec1: Vec<i32> = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, vec2));
}
