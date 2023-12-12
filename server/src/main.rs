use crate::damon::ApphubDamon;
use std::{error::Error, future::pending};

mod damon;
mod imp;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let damon = ApphubDamon::new().await;
    let _con = zbus::ConnectionBuilder::session()?
        .name("dev.bedsteler20.ApphubDamon")?
        .serve_at("/dev/bedsteler20/ApphubDamon", damon)?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}
