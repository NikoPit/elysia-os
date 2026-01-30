use crate::{print, println};

#[cfg(test)]
pub fn run_tests(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

pub struct Test {
    name: &'static str,
    test: fn(),
}

impl Test {
    pub fn new(name: &'static str, test: fn()) -> Self {
        Self { name, test }
    }

    fn run_test(&self) {
        print!("Testing {} ", self.name);

        ((self.test)());

        println!("[OK]");
    }
}

#[macro_export]
macro_rules! test {
    ($name:literal, $test_fn: expr) => {
        #[test_case]
        fn __test() {
            $crate::testing::Test::new($name, $test_fn).run_test();
        }
    };
}

test!("1 = 1?", || assert_eq!(1, 1));
test!("1 + 1 = 2", || assert_eq!(1 + 1, 2));
