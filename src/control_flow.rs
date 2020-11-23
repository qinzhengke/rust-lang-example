fn main(){
    let num = 3;
    if num < 5 {
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }

    // Compile failed, non-bool types cannot be used in condition.
    // if num {
    //     println!("condition of num");
    // }

    // Compile failed assignment is invalid in condition.
    // let mut num = 3;
    // if num = 3 {
    //     println!("expression nuk=3 is true");
    // }

    let num = 6;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if num % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if num %2 == 0 {
        println!("number is divisible by 2");
    }
    else{
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using "if" in let statement.
    let f = true;
    let num = if f { 5 } else { 6 };   // No ";" in "{}"
    println!("num={}", num);

    // Compile failed! "if-assignment" only available for object with the same type.
    // let num = if f { 5 } else { "six" };

    // The loop may has a result and can be returned.
    let mut cnt = 0;
    let res = loop {
        cnt += 1;
        if cnt >= 10 {
            break cnt * 2;  // Break and return result.
        }
    };
    println!("res = {}", res);

    // Compiled failed! "break-return" only available for "loop"
    // let res = while true {
    //     break cnt * 2;  // Break and return result.
    // };

    let mut cnt = 3;
    while cnt >0 {
        println!("cnt = {}", cnt);
        cnt -= 1;
    }
    println!("loop end");

    // Looping through an array with "for"
    let x = [3, 4, 5];
    for ele in x.iter(){
        println!("ele = {}", ele);
    }

    // Compile failed, iter() is necessary for an array.
    // for ele in x{
    //     println!("ele = {}", ele);
    // }

    // Looping though a tuple with "for"
    for n in (1..4).rev(){
        println!("n = {}", n);
    }

}