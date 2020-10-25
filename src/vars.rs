fn main(){
    let name = "Bob";
    let mut age = 18;

    println!("My name is {}, and I am {}", name, age);

    // Reassignment for immutable variable, compile fail!
    // name = "Alan";

    age = 19;
    println!("One year later, I am {}", age);

    // Multiple variables assignments
    let (name, age) = ("Bob", 18);
    println!("My name is {} and I am {}", name, age);

}