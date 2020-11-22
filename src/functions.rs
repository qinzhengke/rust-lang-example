fn main(){
    
    // No need to be defined "before" caller.
    func_after_main();

    print(10, 3.14);

    println!("get_var_10()={}", get_var_10());

    println!("get_var_20()={}", get_var_20());

    println!("plus_one(5)={}", plus_one(5));
}

fn func_after_main(){
    println!("func_after_main()");
}

fn print(x: i32, y: f64){
    println!("x={},y={}", x, y);
}

fn get_var_10() -> i32{
    return 10;
}

fn get_var_20() -> i32{
    // Return last expression by default. No ";" at the end.
    20
}

fn plus_one(x: i32) -> i32{
    // No ";" at the end.
    x+1 
}