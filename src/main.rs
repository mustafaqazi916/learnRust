use std::io;

fn main() {
    println!("Enter name");
    let mut name = String::new();
    io::stdin()
      .read_line(&mut name)
      .expect("failed to read name");

    let name_len = name.len();

    println!("your name is {} and has {} letters", name, name_len);

    
}

