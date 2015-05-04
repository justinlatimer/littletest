extern crate time;

use std::fmt;
use std::io;

#[derive(Clone)]
pub enum TestResult {
    Pass,
    Fail,
    Error,
    Skipped
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = match *self {
            TestResult::Pass => ".",
            TestResult::Fail => "F",
            TestResult::Error => "E",
            TestResult::Skipped => "S"
        };
        write!(f, "{}", rep)
    }
}

pub trait Runnable {
    fn run(&self) -> TestResult;
}

struct TestTimings {
    time_s: f64,
    runs_per_s: f64
}

impl fmt::Display for TestTimings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Finished in {}s, {} runs/s.", self.time_s, self.runs_per_s)
    }
}

struct TestStats {
    runs: usize,
    failures: usize,
    errors: usize,
    skips: usize,
}

impl TestStats {
    fn new() -> TestStats {
        TestStats {
            runs: 0,
            failures: 0,
            errors: 0,
            skips: 0
        }
    }

    fn create(result: &TestResult) -> TestStats {
        TestStats {
            runs: 1,
            failures: match *result { TestResult::Fail => 1, _ => 0 },
            errors: match *result { TestResult::Error => 1, _ => 0 },
            skips: match *result { TestResult::Skipped => 1, _ => 0 },
        }
    }

    fn combine(a: TestStats, b: TestStats) -> TestStats {
        TestStats {
            runs: a.runs + b.runs,
            failures: a.failures + b.failures,
            errors: a.errors + b.errors,
            skips: a.skips + b.skips
        }
    }
}

impl fmt::Display for TestStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} runs, {} failures, {} errors, {} skips.", self.runs, self.failures, self.errors, self.skips)
    }
}

trait Reporter {
    fn start(&mut self);
    fn record(&mut self, result: &TestResult);
    fn report(&mut self);
}

struct ProgressReporter {
    output: io::Stdout
}

impl ProgressReporter {
    fn new() -> ProgressReporter {
        ProgressReporter {
            output: io::stdout()
        }
    }
}

impl Reporter for ProgressReporter {
    fn start(&mut self) {}
    fn record(&mut self, result: &TestResult) {
        use std::io::{Write};
        match write!(&mut self.output, "{}", result) {
            Ok(_) => match self.output.flush() {
                Ok(_) => {},
                _ => panic!("Unable to flush test result")
            },
            _ => panic!("Unable to write test result")
        };
    }
    fn report(&mut self) {}
}

struct StatisticsReporter {
    start_time: u64,
    end_time: u64,
    results: Vec<TestResult>
}

impl StatisticsReporter {
    fn new() -> StatisticsReporter {
        StatisticsReporter {
            start_time: 0,
            end_time: 0,
            results: Vec::new()
        }
    }
}

impl Reporter for StatisticsReporter {
    fn start(&mut self) {
        println!("# Running.\n");
        self.start_time = time::precise_time_ns();
    }
    fn record(&mut self, result: &TestResult) {
        self.results.push(result.clone());
    }
    fn report(&mut self) {
        self.end_time = time::precise_time_ns();

        let time_ms = self.end_time.wrapping_sub(self.start_time) as i64 / 1000000;
        let time_s = time_ms as f64 / 1000f64;
        let timings = TestTimings {
            time_s: time_s,
            runs_per_s: self.results.len() as f64 / time_s
        };

        println!("\n\n{}", timings);

        let stats = self.results
            .iter()
            .map(TestStats::create)
            .fold(TestStats::new(), TestStats::combine);

        println!("\n{}", stats);

        if stats.skips > 0 {
            println!("\nYou have skipped tests. Run with --verbose for details.");
        }
    }
}

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
