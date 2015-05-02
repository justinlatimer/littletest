extern crate time;

use std::fmt;

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

pub struct TestRunner;

impl TestRunner {
    pub fn run(&self, tests: &Vec<Box<Runnable>>) {
        use std::io::{Write,stdout};

        println!("# Running.\n");

        let start_time = time::precise_time_ns();

        let mut results: Vec<TestResult>;

        {
            let out = stdout();
            let mut lock = out.lock();

            results = tests
                .iter()
                .map(|test| {
                    let result = test.run();
                    match write!(&mut lock, "{}", result) {
                        Ok(_) => match lock.flush() {
                            Ok(_) => {},
                            _ => panic!("Unable to flush test result")
                        },
                        _ => panic!("Unable to write test result")
                    };
                    result
                })
                .collect();
        }

        let end_time = time::precise_time_ns();
        let time_ms = end_time.wrapping_sub(start_time) as i64 / 1000000;
        let time_s = time_ms as f64 / 1000f64;
        let timings = TestTimings {
            time_s: time_s,
            runs_per_s: results.len() as f64 / time_s
        };

        println!("\n\n{}", timings);

        let stats = results
            .iter()
            .map(TestStats::create)
            .fold(TestStats::new(), TestStats::combine);

        println!("\n{}", stats);

        if stats.skips > 0 {
            println!("\nYou have skipped tests. Run with --verbose for details.");
        }
    }
}
