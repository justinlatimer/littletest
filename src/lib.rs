extern crate threadpool;

mod types;
pub use types::{TestResult};

pub trait Runnable {
    fn run(&self) -> TestResult;
}

mod reporters;
use reporters::{Reporter,CompositeReporter,ProgressReporter,StatisticsReporter};

pub struct TestOptions {
    pub parallelism: Option<u32>
}

pub struct TestRunner {
    parallelism: u32
}

impl TestRunner {
    pub fn new(options: TestOptions) -> TestRunner {
        match options.parallelism {
            Some(parallelism) => TestRunner {
                parallelism: parallelism
            },
            None => TestRunner {
                parallelism: 1
            }
        }
    }

    pub fn run(&self, tests: &Vec<Box<Runnable + Sync>>) {
        let reporters: Vec<Box<Reporter>> = vec![
            Box::new(ProgressReporter::new()),
            Box::new(StatisticsReporter::new())
        ];
        let mut reporter = CompositeReporter::new(reporters);
        reporter.start();

        {
            let (tx, rx) = std::sync::mpsc::channel();

            let pool = threadpool::ScopedPool::new(self.parallelism);
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
