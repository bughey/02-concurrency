use anyhow::Result;
use concurrency::metrics::Metrics;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    metrics.inc("req.page.1")?;
    metrics.inc("call.thread.worker.1")?;

    println!("{:?}", metrics.snapshot());

    Ok(())
}
