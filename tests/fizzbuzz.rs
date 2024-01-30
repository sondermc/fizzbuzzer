use fizzbuzz;

#[test]
fn fifteen() {
    assert_eq!(
        "FizzBuzz".to_string(),
        fizzbuzz::fizzbuzz(
            &[
                ("Fizz", &|i: i32| i % 3 == 0),
                ("Buzz", &|i: i32| i % 5 == 0)
            ],
            15
        )
    );
}

#[test]
fn five() {
    assert_eq!(
        "Buzz".to_string(),
        fizzbuzz::fizzbuzz(
            &[
                ("Fizz", &|i: i32| i % 3 == 0),
                ("Buzz", &|i: i32| i % 5 == 0)
            ],
            5
        )
    );
}

#[test]
fn three() {
    assert_eq!(
        "Fizz".to_string(),
        fizzbuzz::fizzbuzz(
            &[
                ("Fizz", &|i: i32| i % 3 == 0),
                ("Buzz", &|i: i32| i % 5 == 0)
            ],
            3
        )
    );
}

#[test]
fn one() {
    assert_eq!(
        "1".to_string(),
        fizzbuzz::fizzbuzz(
            &[
                ("Fizz", &|i: i32| i % 3 == 0),
                ("Buzz", &|i: i32| i % 5 == 0)
            ],
            1
        )
    );
}
