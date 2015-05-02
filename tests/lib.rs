extern crate littletest;

use littletest::{Runnable, TestResult, TestRunner};

struct TestCase {
    result: TestResult
}

impl TestCase {
    fn new(result: TestResult) -> TestCase {
        TestCase {
            result: result
        }
    }
}

impl Runnable for TestCase {
    fn run(&self) -> TestResult {
        match self.result {
            TestResult::Pass => TestResult::Pass,
            TestResult::Fail => TestResult::Fail,
            TestResult::Error => TestResult::Error,
            TestResult::Skipped => TestResult::Skipped
        }
    }
}

#[test]
fn it_works() {
    let tests = vec![TestResult::Pass];
    let runnables: Vec<Box<Runnable>> = tests
        .into_iter()
        .map(|result| Box::new(TestCase::new(result)) as Box<Runnable>)
        .collect();

    let runner = TestRunner;
    runner.run(&runnables);
}
