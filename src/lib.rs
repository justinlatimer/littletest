extern crate threadpool;

mod types;
pub use types::{TestResult};

pub trait Runnable {
    fn run(&self) -> TestResult;
}

mod reporters;
use reporters::{Reporter,CompositeReporter,ProgressReporter,StatisticsReporter};

pub struct TestRunner;

impl TestRunner {
    pub fn run(&self, tests: &Vec<Box<Runnable + Sync>>) {
        let reporters: Vec<Box<Reporter>> = vec![
            Box::new(ProgressReporter::new()),
            Box::new(StatisticsReporter::new())
        ];
        let mut reporter = CompositeReporter::new(reporters);
        reporter.start();

        {
            let (tx, rx) = std::sync::mpsc::channel();

            let pool = threadpool::ScopedPool::new(4);
            for test in tests.iter() {
                let tx = tx.clone();
                pool.execute(move|| {
                    let result = test.run();
                    tx.send(result).unwrap();
                });
            }

            for result in rx.iter().take(tests.len()) {
                reporter.record(&result);
            }
        }

        reporter.report();
    }
}
