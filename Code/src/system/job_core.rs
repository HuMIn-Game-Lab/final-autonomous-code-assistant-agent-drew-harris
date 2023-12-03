use serde_json::{from_value, Value};
use anyhow::Result;

use crate::{ai::CompileFix, TestJob};

use super::types::JobType;

pub trait Job {
    fn run(&self) -> Result<Value>;
}

pub fn run_job(job_type: JobType, input: Value) -> Value {
    println!("Running job {:?}", job_type);
    let result = match job_type {
        JobType::CompileFix => {
            let intoed: CompileFix = from_value(input).expect("Valid json");
            intoed.run()
        }
        JobType::Test => {
            let intoed: TestJob = from_value(input).expect("Valid json");
            intoed.run()
        }
        _ => panic!("Job not found"),
    };

    match result {
        Ok(response) => response,
        Err(e) => {
            println!("Error running job: {}", e);
            Value::Null
        }
    }
}
