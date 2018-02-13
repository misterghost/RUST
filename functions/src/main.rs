// we call the functions inside the main function
fn main() {
  print_number(45);
  add_numbers(1234, 543);
  mult_numbers(7654,23);
  add_one(23);
  let f = plus_one;
  let six = f(5);
  let value = chained_function;
  let iva = value(250);
}
// in each function we declare the type of the argument that will be received
// simple function that prints a number:
fn print_number(x:i32) {                // will get an argument of type integer of 32 bits
    println!("X is: {}", x);            // print said argument using string interpolation
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
// the arrow indicates that an integer of 32 bits will be returned
   t + 1                                // yes, without a semicolon. 
}                                       // what happens to the returned value?

// Declaration Statements:
// in ruby you can state: x = y = 5
// but it is not valid in rust
// first, assign a variable, and consider it to be mutable:
//let mut y = 5;
// trying to do as in ruby, you would do:
//let x = (let y = 5);
// but it would get a compile error.
// this wont work either, since in rust, the value of an assigment 
// is an empty tuple, (empty parenthesis) because it can only have one owner
//let x = (y = 6);
// Expression Statements:
// rust expects statements to follow other statements
// and separates expressions from each other by using semicolons
// going to line 25:
// that function returns an i32, but using a semicolon
// it returns (empty parenthesis)

// Early returns:
// using a return as the last line of a function works, but its
// considered poorly written code
fn foo(x: i32) -> i32{
  return x + 1;
}

// Diverging functions:
// These are functions that do not return

fn diverges() -> ! {
  panic!("this function does not return");
}
// panic! is a macro, displays the text on the compiler error message

// Function Pointers:
// f is a variable binding, points to a function that 
// takes an i32 and returns an i32
//let f: fn(i32) -> i32:
// see lines 7 and 8
fn plus_one(i: i32) -> i32 {
  i + 1
}
// we have a function called plus_one, then we bind another
// variable to said function (as in line 7), then we can 
// bind another variable to a function via the first binded
// variable (as in line 8)
// another example: (see lines 9 & 10)
fn chained_function(y: i32) -> i32 {
  y * 16
}


















