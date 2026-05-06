use rp::run;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    run()?;
    Ok(())
}
