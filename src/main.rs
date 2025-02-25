use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    date: String, // YYYY-MM-DD
    completed: bool,
}

fn main() {
}
