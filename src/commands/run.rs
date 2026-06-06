use crate::{errors::CLIError, tools::migrations::run_pending_migrations};

pub async fn run_command() -> Result<(), CLIError> {
    run_pending_migrations().await?;

    Ok(())
}
