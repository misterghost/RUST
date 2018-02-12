fn main() {
    // binding:
    // x is a binding of type 32 bits, and the number 5
    let x :i32 = 5;
    // using patterns to binding multiple values
    let (a, b) = (3, 6);
    // another binding to the result of an operation
    let result = (a + b) * x;
    // print result using interpolation
    // text, curly, variable to go inside curly
    println!("The result is:  {}", result);

    // mutability:
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

    // initialization:
    // cant use a variable that has not been initiated
    // this will give an error since you have specified
    // that e will be an integer of 16 bits, but its 
    // value is not specified
    let e: i16;
    // commented line with updated value (initiated variable)
    // let e: i16 = 245;
    println!("The value of e is: {}", e);

    // scope:
    // we are inside the MAIN's scope
    let h: i32 = 17;
    {
        let g: i32 = 3;
        println!("value of h is {} and value of g is {}", g, h );
        // this works because we are in the inside scope
        // and we have access to both G and H
    }
    println!("value of h is {} and value of g is {}", g, h );
    // this wont work since we are outside of the scope for g:
    // error-> "Cannot find value of G in this scope"

    // variable bindings can be SHADOWED:
    let s: i32 = 8;
    {
        println!("{}", s);      // this prints "8"
        let s = 12;
        println!("{}", s);      // this prints "12"
    }
    println!("{}", s);          // this prints "8"
    let s = 42;
    println!("{}", s);          // this prints "42"

    // shadowing can alter the type 
    let mut w: i32 = 23;        // w is a mutable integer
    // then we change w
    w = 76;                     // now w is immutable, & bound to 76
    let m = 4;                  // m is an integer
    let m = "i can be text";    // now m is of different type



}
  


  
