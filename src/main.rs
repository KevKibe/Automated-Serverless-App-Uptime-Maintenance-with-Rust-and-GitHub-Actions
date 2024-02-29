use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = vec![
        "https://insightai.onrender.com/",
        "https://insightaiserver.onrender.com/health",
        "https://aipredictivecare-backend-services.onrender.com",
        "https://clean-waste-backend.onrender.com",
        "https://debtscleared-backend.onrender.com",
        "https://insightai-backend-eetx.onrender.com",
        "https://insightaiserver-z4ms.onrender.com/health"

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