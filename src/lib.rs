extern crate rayon;

use rayon::prelude::*;
use rayon::par_iter::collect;

mod types;
pub use types::TestResult;

pub trait Runnable {
    fn run(&self) -> TestResult;
}

mod reporters;
use reporters::{Reporter, CompositeReporter, ProgressReporter, StatisticsReporter};

pub struct TestRunner {
    parallelism: bool,
}

impl TestRunner {
    pub fn new(parallelism: bool) -> TestRunner {
        TestRunner { parallelism: parallelism }
    }

    pub fn run(&self, tests: &Vec<Box<Runnable + Sync>>) {
        let reporters: Vec<Box<Reporter>> = vec![Box::new(ProgressReporter::new()),
                                                 Box::new(StatisticsReporter::new())];
        let mut reporter = CompositeReporter::new(reporters);
        reporter.start();

        if self.parallelism {
            let mut results = Vec::with_capacity(tests.len());
            collect::collect_into(tests.into_par_iter()
                                      .map(move |test| test.run()),
                                  &mut results);

            for result in results {
                reporter.record(&result);
            }
        } else {
            for result in tests.into_iter().map(|test| test.run()) {
                reporter.record(&result);
            }
        }

        reporter.report();
    }
}
