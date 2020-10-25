fn main(){
    // Default is "i32"
    let a = 10;

    // Default is "f64"
    let b = 2.5;

    // Add explicit type
    let mut c:i8 = -10;

    println!("a={}, b={}, c={}", a, b, c);

    // Compile fail!
    // c = 128;

    // Still compile fail! The compiler is so smart!
    // let tmp = 128;
    // c = tmp;

    let mut d:u8 = 10;
    
    // Compile fail!
    // d = -1;

    // Number limit
    println!("i32 Max:{}, Min:{}", std::i32::MAX, std::i32::MIN);

    println!("i64 Max:{}, Min:{}", std::i64::MAX, std::i64::MIN);

    println!("f32 Max:{}, Min:{}", std::f32::MAX, std::f32::MIN);


    // Boolean
    let e = true;
    println!("Boolean swi:{}", e);

    let f = 10 > 20;
    println!("f:{}", f);

    // Character
    let g = 'a';
    println!("g:{}", g);

    // Compile fail, u8 is not a character.
    // let u8 g= 'a';

    let h = '\u{1F600}';
    println!("h:{}", h);

    println!("{:?}", (a,b,c,d,e));


}