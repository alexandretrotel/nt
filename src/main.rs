use ntw::dispatch::dispatch;

#[cfg(target_os = "macos")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dispatch().await
}

#[cfg(not(target_os = "macos"))]
fn main() {
    compile_error!("ntw is only supported on macOS (for now?).");
}
