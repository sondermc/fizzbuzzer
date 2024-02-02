pub mod fizzbuzzer;

#[cfg(test)]
mod tests {
    use fizzbuzzer;
    use super::*;

    #[test]
    fn test_1_eq_one() {
        let expected = "one";
        let actual = fizzbuzzer::fizzbuzzer(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2_eq_one() {
        let expected = "two";
        let actual = fizzbuzzer::fizzbuzzer(2);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_modulo_3_eq_fizz() {
        let expected = "fizz";
        let actual = fizzbuzzer::fizzbuzzer(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_modulo_3_not_eq_three() {
        let notexpected = "three";
        let actual = fizzbuzzer::fizzbuzzer(3);
        assert_ne!(actual, notexpected);
    }

    #[test]
    fn test_modulo_5_eq_five() {
        let expected = "buzz";
        let actual = fizzbuzzer::fizzbuzzer(5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_modulo_5_not_eq_five() {
        let notexpected = "five";
        let actual = fizzbuzzer::fizzbuzzer(5);
        assert_ne!(actual, notexpected);
    }

    #[test]
    fn test_modulo_15_eq_fizzbuzz() {
        let expected = "fizzbuzz";
        let actual = fizzbuzzer::fizzbuzzer(15);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_modulo_15_not_eq_fifteen() {
        let notexpected = "fifteen";
        let actual = fizzbuzzer::fizzbuzzer(15);
        assert_ne!(actual, notexpected);
    }
}

