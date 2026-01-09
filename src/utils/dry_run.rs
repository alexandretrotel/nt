use crate::cli::DryRun;
use anyhow::Result;

pub trait DryRunable {
    fn run_with_message(&self, dry_run: DryRun, message: &str) -> Result<()>;
}

impl<F> DryRunable for F
where
    F: Fn() -> Result<()>,
{
    fn run_with_message(&self, dry_run: DryRun, message: &str) -> Result<()> {
        if dry_run.enabled {
            println!("[dry-run] {message}");
            Ok(())
        } else {
            (self)()
        }
    }
}
