use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            check_if_migrations_table_exists,
            get_clickhouse_client_and_ping,
            get_migrations_from_clickhouse,
            undo_migration,
        },
        migrations_operators::get_migrations_from_dir,
    },
    tools::migrations::SetupArgs,
};

pub async fn revert_commmand() -> Result<(), CLIError> {
    let config = SetupArgs::from_toml_file().await?;
    let client = get_clickhouse_client_and_ping(config).await?;

    if !(check_if_migrations_table_exists(client.clone()).await?) {
        return Err(CLIError::InternalError("Migrations table does not exist".to_string()));
    }

    let applied = get_migrations_from_clickhouse(client.clone()).await?;
    let last_applied = applied
        .last()
        .ok_or(CLIError::InternalError("No migrations to revert".to_string()))?;

    let local = get_migrations_from_dir().await?;
    let migration = local
        .into_iter()
        .find(|m| m.version == last_applied.version)
        .ok_or(CLIError::InternalError(format!(
            "Migration {} is applied in DB but not found on disk",
            last_applied.version
        )))?;

    undo_migration(client.clone(), migration).await?;
    Ok(())
}
