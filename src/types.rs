pub fn run() {
  // Default is "i32"
  let _x = 1;

  // Default is "f64"
  let _y = 2.5;

  // Explicitly Define Type
  let _z: i64 = 31415192839;

  // Find MAX_SIZE
  println!("Maximum Value of i32: {}", std::i32::MAX);
  println!("Maximum Value of i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get Boolean from an Expression
  let is_greater: bool = 10 < 5;

  // Character
  let a = 'a';
  let face = '\u{1F600}';

  // Print Everything!
  println!("{:?}", (_x, _y, _x, is_active, is_greater));
  println!("{:?}", (a, face));
}
