fn main() {
  print_number(45);
  add_numbers(1234, 543);
  mult_numbers(7654,23);
}

// simple function that prints a number:
fn print_number(x:i32) {
    println!("X is: {}", x);
}

// simple function to add 2 numbers:
fn add_numbers(a: i32, b: i32) {
    println!("sum is: {}", a + b );
}

// simple function to multiply 2 numbers: 
fn mult_numbers(c: i32, d: i32) {
  println!("multiplication is equal to: {}", c * d);
}

// simple function that adds one to an integer:
fn add_one(t: i32) -> i32 {
  
}
// we call the functions inside the main function
// cargo build, cargo run
// in each function we declare the type of the argument

