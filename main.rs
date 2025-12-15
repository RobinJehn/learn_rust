fn main() {
    println!("Hello world!");

    let remainder = -5 % 3;

    let modu = -5_i32.rem_euclid(3);
    let modu_with_brackets = (-5_i32).rem_euclid(3);

    println!("{remainder} {modu} {modu_with_brackets}");
}
