use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = vec![
        "https://aipredictivecare-backend-services.onrender.com",
        "https://clean-waste-backend.onrender.com",
        "https://debtscleared-backend.onrender.com",
        "https://insightai-backend-eetx.onrender.com",
        "https://insightaiserver-z4ms.onrender.com/health",
        "https://africantrade-backend.onrender.com",
        "https://headhuntrs-backend.onrender.com",
        "https://debtscleared-backend.onrender.com",
        "https://djembeley-backend.onrender.com",
        "https://twintech-backend.onrender.com/",
        "https://sokoly-backend.onrender.com/",
        "https://wellnz-backend.onrender.com/",
        "https://mudmasters-backend.onrender.com/",
        "https://mindmysteries-backend.onrender.com/",
        "https://aipredictivecareserver.onrender.com/health"
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