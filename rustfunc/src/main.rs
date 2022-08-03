fn main() {
    let x: i32 = sum(10, 20);
    println!("{}", x);
    let y: (i32, i32, i32) = sum_v2(10, 20);
    println!("{}", y.0);
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sum_v2(a: i32, b: i32) -> (i32, i32, i32) {
    return (a, b, a + b);
}
