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

#[cfg(test)]
#[macro_use]
extern crate quickcheck;


#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;

    quickcheck! {
        fn modulo_5_contains_buzz(x: usize) -> TestResult {
            if x % 5 != 0 {
                return TestResult::discard()
            }
            TestResult::from_bool(fizz_buzz_string(x).contains("Buzz"))
        }
    }

    quickcheck! {
        fn modulo_3_contains_fizz(x: usize) -> TestResult {
            if x % 3 != 0 {
                return TestResult::discard()
            }
            TestResult::from_bool(fizz_buzz_string(x).contains("Fizz"))
        }
    }

    quickcheck!{
        fn non_divisible_never_contains_fizz_or_buzz(x: usize) -> TestResult{
            if x % 3 == 0 || x % 5 == 0 {
                return TestResult::discard()
            }
            TestResult::from_bool(
                !fizz_buzz_string(x).contains("Fizz") && !fizz_buzz_string(x).contains("Buzz")
            )
        }
    }

}
