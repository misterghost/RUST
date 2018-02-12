fn main() {
    // x is a binding of type 32 bits, and the number 5
    let x :i32 = 5;
    // using patterns to binding multiple values
    let (a, b) = (3, 6);
    // another binding to the result of an operation
    let result = (a + b) * x;
    // print result using interpolation
    // text, curly, variable to go inside curly
    println!("The result is:  {}", result);
    // mutability
    // this will give an error, because we are changing 
    // the value of y without stating it beforehand
    // let y = 5;
    //      y = 45;
    //we state that a variable will have mutability
    // by using mut
    let mut q = 34;
    q = 268; 
    // lets use it so it gives no warning:   
    println!("The value of q is: {}", q);
    // still gives a warning since we stated
    // that it was mutable but we havent mutated it

    // cant use a variable that has not been initiated
    // this will give an error since you have specified
    // that e will be an integer of 16 bits, but its 
    // value is not specified
    let e: i16;
    // commented line with updated value (initiated variable)
    // let e: i16 = 245;
    println!("The value of e is: {}", e);






}
  


  
