fn main() {
    let tup = (10, 'a', 10.5);
    let mut firstelement: i32 = tup.0;
    firstelement = 2;
    println!("{}", firstelement);
    let mut ARR: [i32; 2] = [10, 20];
    println!("{}", ARR[0]);
}
