#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn create(width: u32, height: u32) -> Self {
    Self { width, height }
  }
  // &をつけると、所有権が移動しない 参照させる
  fn area(&self) {
    println!("{}", self.width * self.height);
  }
}

pub fn run() {
  let user1 = User {
    email: String::from("someone@mail.com"),
    username: String::from("someone"),
    sign_in_count: 1,
    active: true,
  };

  let mut user1 = User {
    email: String::from("someone@mail.com"),
    username: String::from("someone"),
    sign_in_count: 1,
    active: true,
  };

  user1.email = String::from("ryosk7@mail.com");
  println!("{:#?}", user1);
  let user2 = build_user(String::from("ryosk7"), String::from("hoge@mail.com"));
  println!("{:#?}", user2);

  let rect = Rectangle::create(20, 20);
  println!("{:#?}", rect);
  rect.area();
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
