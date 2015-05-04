extern crate time;

use types::{TestResult,TestTimings,TestStats};

use std::io;

pub trait Reporter {
    fn start(&mut self);
    fn record(&mut self, result: &TestResult);
    fn report(&mut self);
}

pub struct CompositeReporter {
    reporters: Vec<Box<Reporter>>
}

impl CompositeReporter {
    pub fn new(reporters: Vec<Box<Reporter>>) -> CompositeReporter {
        CompositeReporter {
            reporters: reporters
        }
    }
}

impl Reporter for CompositeReporter {
    fn start(&mut self) {
        for reporter in self.reporters.iter_mut() {
            reporter.start();
        }
    }
    fn record(&mut self, result: &TestResult) {
        for reporter in self.reporters.iter_mut() {
            reporter.record(result);
        }
    }
    fn report(&mut self) {
        for reporter in self.reporters.iter_mut() {
            reporter.report();
        }
    }
}

pub struct ProgressReporter {
    output: io::Stdout
}

impl ProgressReporter {
    pub fn new() -> ProgressReporter {
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

pub struct StatisticsReporter {
    start_time: u64,
    end_time: u64,
    results: Vec<TestResult>
}

impl StatisticsReporter {
    pub fn new() -> StatisticsReporter {
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
