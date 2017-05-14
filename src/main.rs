fn main() {
    let infinite_fizz_buzz = (1..).map(fizz_buzz_string).map(|x| println!("{:?}", x));

    infinite_fizz_buzz.take(10000000).fold((), |_, _| ());
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
