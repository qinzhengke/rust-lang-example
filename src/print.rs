fn main(){
    println!("Hello World!");

    println!("Print numbers: a={},b={},c={}", 1024, 10.0/3.0, true);

    // Positional arguments
    println!("{0} is {1} and {0} likes to play {2}", "Bob", 18, "basketball");

    // Named arguments
    println!("My name is {name}, my age is {age}", name="Bob", age=18);

    // Placeholder trais.
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder debug trais.
    println!("Debug formatter: {:?}", (12, true, "hello"));
    
}