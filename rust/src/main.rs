// src/main.rs
use tokio_postgres::{NoTls, Error};

async fn batch_insert(batch_size: usize, total_count: usize) -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=user password=password dbname=benchmark_db", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    for i in (0..total_count).step_by(batch_size) {
        let params: Vec<String> = (0..batch_size.min(total_count - i))
            .map(|j| format!("Row {}", i + j + 1))
            .collect();
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params.iter().map(|s| s as _).collect();

        let values: Vec<String> = (0..params.len()).map(|idx| format!("(${})", idx + 1)).collect();
        let query = format!("INSERT INTO benchmark (value) VALUES {}", values.join(","));

        let timer = std::time::Instant::now();
        client.execute(&query, &param_refs).await?;
        println!("Insert Batch {}: {:?}", i / batch_size + 1, timer.elapsed());
    }

    Ok(())
}

async fn clear_records() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=user password=password dbname=benchmark_db", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client.execute("DELETE FROM benchmark", &[]).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    batch_insert(100, 6000000).await?;
    println!("Batch insert complete");
    Ok(())
}

// ./target/debug/rust  10.78s user 3.60s system 40% cpu 35.311 total
// node node/benchmark.js  3.24s user 1.66s system 21% cpu 23.083 total

// ./rust  2.33s user 3.59s system 19% cpu 29.791 total
// ./rust  2.32s user 3.67s system 20% cpu 29.233 total

// node benchmark.js  3.08s user 1.61s system 19% cpu 23.480 total
// node benchmark.js  3.11s user 1.56s system 19% cpu 23.511 total