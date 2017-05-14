fn main() {
    let infinite_fizz_buzz = (1..).map(fizz_buzz_string);
    let strings: Vec<String> = infinite_fizz_buzz.take(100).collect();

    for x in strings {
        printstr(x)
    }
}

fn printstr(x: String) -> () {
    println!("{:?}", x)
}

fn fizz_buzz_string(x: usize) -> String {
    if (x % 3 == 0) && (x % 5 == 0) {
        "FizzBuzz".to_string()
    } else if x % 3 == 0 {
        "Fizz".to_string()
    } else if x % 5 == 0 {
        "Buzz".to_string()
    } else {
        x.to_string()
    }
}
