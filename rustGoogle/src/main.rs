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

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    variables(10);
    println!("result {}", arithmatic(10,20));

    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
}
