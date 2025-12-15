fn main() {
    let cel = 38.0;
    let fahr = cel_to_fahr(cel);
    println!("{cel}° is {fahr} Fahrenheit!");

    println!("Fib(10)={}", fib(10));

    print_christmas_carol();
}

fn cel_to_fahr(cel: f64) -> f64 {
    cel * 1.8 + 32.0
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

fn num_to_day(n: usize) -> &'static str {
    match n {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eigth",
        8 => "ninth",
        9 => "tenth",
        10 => "eleventh",
        11 => "twelth",
        _ => "unknown",
    }
}

fn print_christmas_carol() {
    println!("");
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..gifts.len() {
        let day = num_to_day(i);
        println!("On the {day} day of Christmas");
        println!("My true love gave to me");

        for k in 0..=i {
            println!("{}", gifts[i - k]);
        }
        println!();
    }
}
