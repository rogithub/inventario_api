use inventario_api::{App, Result};

#[tokio::main]
async fn main() -> Result<()> {
    App::run().await
}
