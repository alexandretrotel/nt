use crate::cli::NetworkAction;
use anyhow::Result;

mod list;
mod remove;

pub async fn run(action: NetworkAction) -> Result<()> {
    match action {
        NetworkAction::List { iface, dry_run } => list::run(&iface, dry_run),
        NetworkAction::Remove {
            iface,
            ssid,
            dry_run,
        } => remove::run(&iface, &ssid, dry_run).await,
    }
}
