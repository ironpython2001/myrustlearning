fn main() {
    let marks: i32 = 56;
    if marks >= 90 {
        println!("grade a");
    } else {
        println!("grade b");
    }

    let mut i: i32 = 0;
    let result = loop {
        i = i + 1;
        println!("{}", i);
        if i == 3 {
            break 2 * i;
        }
    };

    println!("{}", result);
}
