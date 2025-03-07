use lazy_static::lazy_static;
use log::{trace, warn};
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PlaintextResult {
    pub text: String,
    pub description: String,
    pub checker_name: String,
}

lazy_static! {
    static ref PLAINTEXT_RESULTS: Mutex<Vec<PlaintextResult>> = Mutex::new(Vec::new());
}

pub fn add_plaintext_result(text: String, description: String, checker_name: String) {
    let result = PlaintextResult {
        text: text.clone(),
        description: description.clone(),
        checker_name: checker_name.clone(),
    };

    trace!("Adding plaintext result: [{}] {}", checker_name, text);

    let mut results = match PLAINTEXT_RESULTS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!("Mutex was poisoned, recovering");
            poisoned.into_inner()
        }
    };

    results.push(result);
    trace!("Storage now has {} results", results.len());
}

pub fn get_plaintext_results() -> Vec<PlaintextResult> {
    let results = match PLAINTEXT_RESULTS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!("Mutex was poisoned, recovering");
            poisoned.into_inner()
        }
    };

    trace!("Retrieving {} plaintext results", results.len());
    results.clone()
}

pub fn clear_plaintext_results() {
    let mut results = match PLAINTEXT_RESULTS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!("Mutex was poisoned, recovering");
            poisoned.into_inner()
        }
    };

    trace!("Clearing plaintext results (had {} results)", results.len());
    results.clear();
}
