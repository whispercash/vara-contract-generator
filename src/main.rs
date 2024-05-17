mod cli;
use cli::Cli;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::read_args();
    Ok(())
}
