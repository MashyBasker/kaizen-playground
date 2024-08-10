use testdata::calculator_rust::Calculator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_initialization() {
        let calc = Calculator::new();
        assert_eq!(calc.value, 0, "Calculator should be initialized with value 0");
    }

    #[test]
    fn test_calculator_addition() {
        let mut calc = Calculator::new();
        calc.add(5);
        assert_eq!(calc.value, 5, "Calculator should correctly add values");
    }

    #[test]
    fn test_calculator_subtraction() {
        let mut calc = Calculator::new();
        calc.add(10);
        calc.subtract(3);
        assert_eq!(calc.value, 7, "Calculator should correctly subtract values");
    }

    #[test]
    fn test_calculator_multiplication() {
        let mut calc = Calculator::new();
        calc.add(2);
        calc.multiply(4);
        assert_eq!(calc.value, 8, "Calculator should correctly multiply values");
    }

    #[test]
    fn test_calculator_division() {
        let mut calc = Calculator::new();
        calc.add(8);
        calc.divide(2);
        assert_eq!(calc.value, 4, "Calculator should correctly divide values");
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_calculator_division_by_zero() {
        let mut calc = Calculator::new();
        calc.divide(0);
    }

    #[test]
    fn test_calculator_reset() {
        let mut calc = Calculator::new();
        calc.add(10);
        calc.reset();
        assert_eq!(calc.value, 0, "Calculator should reset to 0");
    }
}