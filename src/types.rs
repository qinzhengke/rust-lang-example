fn main(){
    // Default is "i32"
    let a = 10;

    // Default is "f64"
    let b = 2.5;

    // Add explicit type
    let mut c:i8 = -10;

    println!("[1] a={}, b={}, c={}", a, b, c);

    let large_num = 1000_000_000;
    println!("[2] large_num = {}", large_num);

    // Complie failed! Numeric overflow
    // c = 128;

    // Complie failed! Still compile fail with a little more complex pipeline!
    // The rust compiler is so smart!
    // let tmp = 128;
    // c = tmp;

    let mut d:u8 = 255;

    // Complie failed! 
    // d += 1;
    
    // Complie failed! Numerical overflow.
    // d = -1;

    // Number limit
    println!("[3] i32 Max:{}, Min:{}", std::i32::MAX, std::i32::MIN);

    println!("[4] i64 Max:{}, Min:{}", std::i64::MAX, std::i64::MIN);

    println!("[5] f32 Max:{}, Min:{}", std::f32::MAX, std::f32::MIN);

    // Boolean
    let e = true;
    println!("[6] Boolean swi:{}", e);

    let f = 10 > 20;
    println!("[7] f:{}", f);

    // Character
    let g = 'a';
    println!("[8] g:{}", g);

    // Complie failed! u8 is not a character.
    // let u8 g= 'a';

    let h = '\u{1F600}';
    println!("[9] h:{}", h);

    println!("[10] {:?}", (a,b,c,d,e));

    // Tuple, use "{:?}" to print all the elements with debug format.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("[11] tup={:?}", tup);

    println!("[12] tup=({},{},{})", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("[13] x={},y={},z={}", x, y, z);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("[14] arr[0]={}, arr[1]={}", arr[0], arr[1]);

    // Compile failed!
    // println!("arr[10]={}", arr[10]);

    // Same with tuple, use "{:?}" to print all elements.
    let arr: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("[15] arr={:?}", arr);

    // Complie failed! float variables cannot be assigned by integer literal.
    // let arr: [f32; 5] = [1, 2, 3, 4, 5];
    // println!("[15] arr={:?}", arr);

    let arr = [3; 5];
    println!("[16] arr={:?}", arr);
}