use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = vec![
        "https://behaviour-analysis.onrender.com/",
        "https://insightai.onrender.com/",
        "https://insightaiserver.onrender.com/health",
        "https://aipredictivecare-backend-services.onrender.com"
    ];

    for url in urls {
        let res = match reqwest::get(url).await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Failed to get '{}': {}", url, e);
                continue;
            }
        };

        println!("Response from {}: {}", url, res);
    }

    Ok(())
}