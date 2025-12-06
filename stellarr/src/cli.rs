use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stellarr")]
#[command(about = "Unified media management application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the Stellarr server
    Serve {
        /// Host to bind to
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Run database migrations
    Migrate,

    /// Test external service connections
    Test {
        #[arg(value_enum)]
        service: TestService,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum TestService {
    Tmdb,
    Database,
    All,
}
