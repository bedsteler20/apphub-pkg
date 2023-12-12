use crate::daemon::ApphubDaemon;
use std::{error::Error, future::pending};

mod daemon;
mod imp;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let damon = ApphubDaemon::new().await;
    let _con = zbus::ConnectionBuilder::session()?
        .name("dev.bedsteler20.ApphubDaemon")?
        .serve_at("/dev/bedsteler20/ApphubDaemon", damon)?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}
