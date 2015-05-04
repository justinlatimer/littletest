use std::fmt;

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

pub struct TestTimings {
    pub time_s: f64,
    pub runs_per_s: f64
}

impl fmt::Display for TestTimings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Finished in {}s, {} runs/s.", self.time_s, self.runs_per_s)
    }
}

pub struct TestStats {
    pub runs: usize,
    pub failures: usize,
    pub errors: usize,
    pub skips: usize,
}

impl TestStats {
    pub fn new() -> TestStats {
        TestStats {
            runs: 0,
            failures: 0,
            errors: 0,
            skips: 0
        }
    }

    pub fn create(result: &TestResult) -> TestStats {
        TestStats {
            runs: 1,
            failures: match *result { TestResult::Fail => 1, _ => 0 },
            errors: match *result { TestResult::Error => 1, _ => 0 },
            skips: match *result { TestResult::Skipped => 1, _ => 0 },
        }
    }

    pub fn combine(a: TestStats, b: TestStats) -> TestStats {
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
