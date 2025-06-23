fn main() {
    /*
    --------- Ownership rules ------
    1. Each valuee in Rust has a variable that's called its owner
    2. There can only be one owner at a time
    3. When the owner goes out of scope, the value will be dropped
     
    --------- Rules of References --
    1. At any given time, you can have either one mutable reference or any number of immutable references.
    2. References must always be valid

     
    */

    let s = String::from("hello");
    take_ownership(s);
    println!("{}, world!", s);
}
fn take_ownership(some_string: String){
    println!("{}", some_string);
}