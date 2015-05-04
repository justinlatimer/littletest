mod types;
pub use types::{TestResult};

pub trait Runnable {
    fn run(&self) -> TestResult;
}

mod reporters;
use reporters::{Reporter,StatisticsReporter};

pub struct TestRunner;

impl TestRunner {
    pub fn run(&self, tests: &Vec<Box<Runnable>>) {
        let mut reporter = StatisticsReporter::new();
        reporter.start();

        let results: Vec<TestResult> = tests
            .iter()
            .map(|test| test.run())
            .map(|result| {
                reporter.record(&result);
                result
            })
            .collect();

        reporter.report();
    }
}
