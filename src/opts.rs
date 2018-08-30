use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Id {
    #[structopt(subcommand)]
    pub id_command: IdCommand,
}

#[derive(Debug, StructOpt, Clone)]
pub enum IdCommand {
    #[structopt(name = "gen")]
    /// Generate a CrevID
    Gen,
    #[structopt(name = "show")]
    /// Show CrevID information
    Show,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Add {
    #[structopt(parse(from_os_str))]
    /// Paths to add
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Remove {
    #[structopt(parse(from_os_str))]
    /// Paths to remove
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, StructOpt, Clone)]
pub struct TrustAdd {
    /// Public IDs to create Trust Proof for
    pub pub_ids: Vec<String>,
}

#[derive(Debug, StructOpt, Clone)]
pub enum Trust {
    #[structopt(name = "add")]
    /// Create a new Trust Proof
    Add(TrustAdd),
}

#[derive(Debug, StructOpt, Clone)]
pub struct Verify {
    depth: u64,
    #[structopt(long = "high-cost")]
    high_cost: u64,
    #[structopt(long = "medium-cost")]
    medium_cost: u64,
    #[structopt(long = "low-cost")]
    low_cost: u64,
}

#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    #[structopt(name = "id")]
    /// CrevID management
    Id(Id),
    #[structopt(name = "add")]
    /// Add paths to reviewed list
    Add(Add),
    /// Create a new Review Proof from reviewed list
    #[structopt(name = "commit")]
    Commit,
    #[structopt(name = "init")]
    /// Init `.crev` directory
    Init,
    #[structopt(name = "status")]
    /// Display pending review list
    Status,
    #[structopt(name = "rm")]
    /// Remove path from reviewed list
    Remove(Remove),
    /// Verify review coverage of the project
    Verify(Verify),
    #[structopt(name = "trust")]
    /// Trust Store management
    Trust(Trust),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "crev", about = "Distributed code review system")]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
    //    #[structopt(flatten)]
    //    verbosity: Verbosity,
}
