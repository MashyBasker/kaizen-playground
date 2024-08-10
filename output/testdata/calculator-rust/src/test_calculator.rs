#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_result_is_zero() {
        let calc = Calculator { result: 0 };
        assert_eq!(calc.result, 0);
    }

    #[test]
    fn test_addition() {
        let mut calc = Calculator { result: 0 };
        calc.result += 5;
        assert_eq!(calc.result, 5);
    }

    #[test]
    fn test_subtraction() {
        let mut calc = Calculator { result: 10 };
        calc.result -= 3;
        assert_eq!(calc.result, 7);
    }

    #[test]
    fn test_multiplication() {
        let mut calc = Calculator { result: 2 };
        calc.result *= 3;
        assert_eq!(calc.result, 6);
    }

    #[test]
    fn test_division() {
        let mut calc = Calculator { result: 10 };
        calc.result /= 2;
        assert_eq!(calc.result, 5);
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        let mut calc = Calculator { result: 10 };
        calc.result /= 0;
    }

    #[test]
    fn test_large_number_addition() {
        let mut calc = Calculator { result: i32::MAX - 1 };
        calc.result += 1;
        assert_eq!(calc.result, i32::MAX);
    }

    #[test]
    fn test_large_number_subtraction() {
        let mut calc = Calculator { result: i32::MIN + 1 };
        calc.result -= 1;
        assert_eq!(calc.result, i32::MIN);
    }
}