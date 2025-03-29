pub mod coordinator;
pub mod node;
pub mod operator;

// Node CLI
#[path = "node/cli/mod.rs"]
pub mod ncli;

// Coordinator CLI
#[path = "coordinator/cli/mod.rs"]
pub mod ccli;

// Operator CLI
#[path = "operator/cli/mod.rs"]
pub mod ocli;
