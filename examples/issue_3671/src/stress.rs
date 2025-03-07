use std::collections::BTreeMap;
use tokio::io::{self, AsyncWriteExt};

pub async fn fetch(
    target: &str,
    count: u32,
) -> (u32, u32, BTreeMap<usize, u32>) {
    let mut stdout = io::stdout();
    let mut ok = 0;
    let mut err = 0;
    let mut sizes = BTreeMap::new();
    for i in 0..count {
        let _ = stdout.write_all(format!("{i}\r").as_bytes()).await;
        let _ = stdout.flush().await;
        if let Ok(result) = reqwest::get(target).await {
            ok += 1;
            if let Ok(bytes) = result.bytes().await {
                sizes
                    .entry(bytes.len())
                    .and_modify(|curr| *curr += 1)
                    .or_insert(1);
            } else {
                sizes.entry(0).and_modify(|curr| *curr += 1).or_insert(1);
            }
        } else {
            err += 1;
        }
    }
    (ok, err, sizes)
}

pub async fn report(target: &str, count: u32) {
    let (ok, err, sizes) = fetch(&target, count).await;
    println!("ok    = {ok}");
    println!("err   = {err}");
    println!("sizes = {sizes:?}");
}
