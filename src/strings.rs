fn main(){

    let a = "Hello";
    println!("{}",a);

    // Compile fail! Primitive string is with fixed length.
    // a.push('!');
    // b.push_str(" Is someone there?");

    // Compile fail! Primitive string has no property of capacity.
    // println!("Capacity:{}", a.capacity());

    let mut b = String::from("Hello");
    println!("{}", b);

    // Capacity in bytes.
    println!("Capacity:{}", b.capacity());

    // Get length
    println!("len:{}", b.len());

    // Append character.
    b.push('!');
    println!("{}",b);

    // Append string
    b.push_str(" Is someone there?");
    println!("{}", b);

    // Capacity in bytes.
    println!("Len:{}, Capacity:{}", b.len(), b.capacity());

    // The capacity is equal to the length, dyanamically.
    b = String::from("Hello!");
    println!("Len:{}, Capacity:{}", b.len(), b.capacity());

}