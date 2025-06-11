use nv_triton_rs::TritonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Triton Client...");
    let client = TritonClient::new("http://localhost:9101").await;
    println!("Triton Client created successfully!");

    let server_live = client.is_server_live().await?;
    let server_ready = client.is_server_ready().await?;
    let models = client.model_repository_index().await?.models;
    let models_description = models
        .iter()
        .map(|model| model.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    println!("Server Status:");
    println!("\tlive: {}", server_live);
    println!("\tready: {}", server_ready);
    println!("\tmodels: {models_description}");

    println!("Loading kokoro_82m...");
    client.load_model(None, "kokoro_82m", None).await?;
    println!("kokoro_82m loaded successfully");

    println!("Unloading kokoro_82m...");
    client.unload_model(None, "kokoro_82m", None).await?;
    println!("kokoro_82m unloaded successfully");

    Ok(())
}
