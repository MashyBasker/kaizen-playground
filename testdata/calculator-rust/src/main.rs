struct Calculator {
    result: i32,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator { result: 0 }
    }

    fn add(&mut self, a: i32, b: i32) -> i32 {
        self.result = a + b;
        self.result
    }

    fn subtract(&mut self, a: i32, b: i32) -> i32 {
        self.result = a - b;
        self.result
    }

    fn get_result(&self) -> i32 {
        self.result
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let mut calc = Calculator::new();
    println!("{}", calc.add(5, 3));  // Should print 8
    println!("{}", calc.subtract(10, 4));  // Should print 6
    println!("{}", calc.get_result());  // Should print 6
    println!("{}", greet("Alice"));  // Should print "Hello, Alice!"
}
