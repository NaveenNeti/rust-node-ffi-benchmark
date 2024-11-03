
use tokio::task;
use tokio_postgres::{NoTls, Error};
use std::sync::Arc;
use futures::future::join_all;

async fn batch_insert_async(thread_count: usize, total_count: usize) -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=user password=password dbname=benchmark_db",
        NoTls,
    ).await?;

    // Spawn a task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let client = Arc::new(client);
    let mut handles = Vec::new();

    for i in 0..total_count {
        let client = Arc::clone(&client);
        let handle = task::spawn(async move {
            let param = format!("Row {}", i + 1);
            let query = "INSERT INTO benchmark (value) VALUES ($1)";
            let timer = tokio::time::Instant::now();
            client.execute(query, &[&param]).await?;
            println!("Insert Row {}: {:?}", i + 1, timer.elapsed());
            Ok::<(), Error>(())
        });

        handles.push(handle);

        if handles.len() == thread_count {
            // Await the current batch
            let results = join_all(handles).await;
            for result in results {
                if let Err(e) = result {
                    eprintln!("Task error: {}", e);
                }
            }
            handles = Vec::new();
        }
    }

    // Await any remaining tasks
    if !handles.is_empty() {
        let results = join_all(handles).await;
        for result in results {
            if let Err(e) = result {
                eprintln!("Task error: {}", e);
            }
        }
    }

    Ok(())
}

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    batch_insert_async(10, 200000).await?;
    println!("Batch insert complete");
    Ok(())
}