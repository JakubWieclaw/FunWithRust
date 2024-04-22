use std::time::Duration;

pub struct HashFunctionResult {
    pub hash_type: String,
    pub time_1mb: Duration,
    pub time_3mb: Duration,
    pub time_10mb: Duration,
}