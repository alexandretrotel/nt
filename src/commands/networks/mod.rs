use crate::cli::NetworkAction;
use anyhow::Result;

mod list;
mod remove;

pub fn run(action: NetworkAction) -> Result<()> {
    match action {
        NetworkAction::List { iface } => list::run(&iface),
        NetworkAction::Remove { iface, ssid } => remove::run(&iface, &ssid),
    }
}
