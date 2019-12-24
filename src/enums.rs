// Enums are types with definitive values

enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  match m {
    Movement::Up => println!("Avatar is going up!"),
    Movement::Down => println!("Avatar is going down!"),
    Movement::Left => println!("Avatar is going left!"),
    Movement::Right => println!("Avatar is going right!"),
  }
}

pub fn run() {
  let avatar1 = Movement::Up;
  let avatar2 = Movement::Down;
  let avatar3 = Movement::Left;
  let avatar4 = Movement::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}
