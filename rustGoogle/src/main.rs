fn arithmatic(a: i32, b:i32)-> i32{
    return a*b;
}

fn variables(a:i32){
    let mut x: i32 = a;
    println!(" x : {x}");
    x = 20;
    println!(" x : {x}");
    return;
}

fn main() {
    variables(10);
    println!("result {}", arithmatic(10,20));
}
