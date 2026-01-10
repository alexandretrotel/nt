use anyhow::Result;

use crate::cli::DryRun;

mod list;
mod remove;

pub fn run_list(iface: &str, dry_run: DryRun) -> Result<()> {
    list::run(iface, dry_run)
}

pub async fn run_remove(iface: &str, ssid: &Option<String>, dry_run: DryRun) -> Result<()> {
    remove::run(iface, ssid, dry_run).await
}
