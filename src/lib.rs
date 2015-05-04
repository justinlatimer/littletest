mod types;
pub use types::{TestResult};

pub trait Runnable {
    fn run(&self) -> TestResult;
}

mod reporters;
use reporters::{Reporter,CompositeReporter,ProgressReporter,StatisticsReporter};

pub struct TestRunner;

impl TestRunner {
    pub fn run(&self, tests: &Vec<Box<Runnable>>) {
        let reporters: Vec<Box<Reporter>> = vec![
            Box::new(ProgressReporter::new()),
            Box::new(StatisticsReporter::new())
        ];
        let mut reporter = CompositeReporter::new(reporters);
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
