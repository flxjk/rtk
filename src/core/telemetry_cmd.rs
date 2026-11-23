use anyhow::{Context, Result};
use clap::Subcommand;

#[cfg(test)]
pub(crate) const TELEMETRY_DISABLED_ENV: &str = "RTK_TELEMETRY_DISABLED";

#[derive(Debug, Subcommand)]
pub enum TelemetrySubcommand {
    Status,
    Enable,
    Disable,
    Forget,
}

pub fn run(command: &TelemetrySubcommand) -> Result<()> {
    match command {
        TelemetrySubcommand::Status => {
            println!("Telemetry status:");
            println!("  enabled:       no");
            println!("External telemetry is disabled in this build.");
            Ok(())
        }
        TelemetrySubcommand::Enable => {
            anyhow::bail!("External telemetry is disabled in this build.")
        }
        TelemetrySubcommand::Disable => {
            record_opt_out()?;
            println!("Telemetry disabled.");
            Ok(())
        }
        TelemetrySubcommand::Forget => forget_local_data(),
    }
}

fn record_opt_out() -> Result<()> {
    let mut config = crate::core::config::Config::load().unwrap_or_default();
    config.telemetry.consent_given = Some(false);
    config.telemetry.enabled = false;
    config.telemetry.consent_date = Some(chrono::Utc::now().to_rfc3339());
    config
        .save()
        .context("Failed to save telemetry opt-out to config.toml")
}

fn forget_local_data() -> Result<()> {
    record_opt_out()?;

    // These names match files written by older builds.
    let telemetry_dir = super::user_dirs::data_under("/tmp");
    let history = super::user_dirs::data_under(".").join(super::constants::HISTORY_DB);
    for path in [
        telemetry_dir.join(".device_salt"),
        telemetry_dir.join(".telemetry_last_ping"),
        history,
    ] {
        match std::fs::remove_file(&path) {
            Ok(()) => println!("Deleted: {}", path.display()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).with_context(|| format!("Failed to delete {}", path.display()));
            }
        }
    }
    println!("Local telemetry data deleted. Telemetry disabled.");
    Ok(())
}
