#![allow(dead_code)]

mod strategy {
    trait Operation {
        fn operation(&self, value_one: i32, value_two: i32) -> i32;
    }

    struct Calculator {
        operation: Box<dyn Operation>,
    }

    impl Calculator {
        fn new(operation: Box<dyn Operation>) -> Self {
            Self { operation }
        }

        fn perform_operation(&self, value_one: i32, value_two: i32) -> i32 {
            self.operation.operation(value_one, value_two)
        }

        fn set_operation(&mut self, operation: Box<dyn Operation>) {
            self.operation = operation;
        }
    }

    struct Addition;

    impl Operation for Addition {
        fn operation(&self, value_one: i32, value_two: i32) -> i32 {
            value_one + value_two
        }
    }

    struct Substract;

    impl Operation for Substract {
        fn operation(&self, value_one: i32, value_two: i32) -> i32 {
            value_one - value_two
        }
    }

    fn _test() {
        let mut calculator = Calculator::new(Box::new(Addition));

        println!("{:?}", calculator.perform_operation(10, 30));

        calculator.set_operation(Box::new(Substract));

        println!("{:?}", calculator.perform_operation(10, 30));
    }

    struct OtherOps<F>
    where
        F: Fn(i32, i32) -> i32,
    {
        operation: F,
    }

    impl<F> OtherOps<F>
    where
        F: Fn(i32, i32) -> i32,
    {
        fn new(operation: F) -> Self {
            Self { operation }
        }

        fn perform_operation(&self, value_one: i32, value_two: i32) -> i32 {
            (self.operation)(value_one, value_two)
        }

        fn set_operation(&mut self, operation: F) {
            self.operation = operation;
        }
    }

    fn _test_two() {
        let calculator = OtherOps::new(|value_one, value_two| value_one + value_two);

        println!("{:?}", calculator.perform_operation(40, 10));

        // calculator.set_operation(|value_one, value_two| value_one - value_two);

        println!("{:?}", calculator.perform_operation(50, 10));
    }

    #[test]
    fn test() {
        _test();

        _test_two()
    }
}

fn main() {
    fn operation(mt_1: i32, mt_2: i32, func: impl Fn(i32, i32) -> i32) -> i32 {
        func(mt_1, mt_2)
    }

    println!(
        "The result is: {}",
        operation(10, 30, |a, b| {
            // Addition
            a + b
        })
    );
}
