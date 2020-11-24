fn main(){
    // s1 and s2 are variable and they are stored on the stack.
    // "String::from("hello")" is a value and stored on the heap.
    // s1 and s2 shared the same data: "Hello".
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2={}", s2);

    // Compiled failed! s1 was moved into s2.
    // println!("s1={}", s1);

    let s1 = String::from("hello2");
    let s2 = s1.clone();
    println!("s2={}", s2);

    // Compile successfully! Unknown size object at compile time store on stack.
    // This is called "Copy trait".
    let x = 5;
    let y = x;
    println!("y={}", y);

    let s1 = String::from("hello");
    take_ownership(s1);

    // Compile failed! s1 lost the owner ship.
    // println!("s1={}", s1);

    let x = 5;
    take_ownership_i32(x);

    // Compile successfully because x:i32 has "Copy trait"
    println!("x={}", x);

    let s1 = String::from("hello");
    pass_by_reference(&s1);
    // Compile successfully! s1 still has the ownership.
    println!("s1={}", s1);

    // Compiled failed! Reference is immutable.
    let mut s1 = String::from("hello");
    change_by_reference(&mut s1);
    println!("s1={}", s1);

    let s1 = gives_ownership();
    println!("s1={}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_ownership(s2);
    println!("s3={}", s3);

    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    // Compile failed! Only one reference is allowed for mutable.
    // let r2 = &mut s1;
    println!("r1={}", r1);
    // Compile successfully! r1 is not longer used, we can change the ref now.
    let r2 = &mut s1;


    // A mutable reference is not compatible with other reference, even for the 
    // immutable references 
    let mut s1 = String::from("hello");
    let r1 = &s1;   // No problem
    let r2 = &s1;   // No problem
    // let r3 = &mut s1;   // Big problem.
    println!("r1={},r2={}", r1, r2);


}

fn take_ownership(s: String){
    println!("s={}", s);
}

fn take_ownership_i32(x: i32){
    println!("x={}",x);
}

fn gives_ownership() -> String{
    let s = String::from("hello");
    s
}

fn takes_and_gives_ownership(s: String) -> String {
    s
}

fn pass_by_reference(s: &String){
    println!("s={}", s);
}

fn change_by_reference(s: &mut String){
    s.push_str(", world");
}