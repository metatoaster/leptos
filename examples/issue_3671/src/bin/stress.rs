#[tokio::main]
async fn main() {
    use issue_3671::stress::report;

    let mut args = std::env::args();
    args.next();
    if let Some(target) = args.next() {
        let count = args
            .next()
            .map(|s| s.parse::<u32>().unwrap_or(1000))
            .unwrap_or(1000);
        report(&target, count).await;
    } else {
        eprintln!("URL required.");
    }
}
