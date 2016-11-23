extern crate littletest;

use littletest::{Runnable, TestResult, TestRunner};

struct TestCase {
    result: TestResult,
}

impl TestCase {
    fn new(result: TestResult) -> TestCase {
        TestCase { result: result }
    }
}

impl Runnable for TestCase {
    fn run(&self) -> TestResult {
        self.result.clone()
    }
}

#[test]
fn it_works() {
    use std::iter::repeat;

    let runnables = repeat(TestResult::Pass)
        .take(10)
        .map(|result| Box::new(TestCase::new(result)) as Box<Runnable + Sync>)
        .collect::<Vec<_>>();

    let runner = TestRunner::new(true);
    runner.run(&runnables);
}
