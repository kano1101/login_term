use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::error!("error message");
    tracing::warn!("warn message");
    tracing::info!("info message");
    tracing::debug!("debug message");
    tracing::trace!("trace message");

    dotenv::dotenv().ok();

    let secrets_manager_id = std::env::var("SECRETS_MANAGER_ID")?;
    let username = std::env::var("COGNITO_USERNAME")?;
    let password = std::env::var("COGNITO_PASSWORD")?;
    let api_gateway_url = std::env::var("API_GATEWAY_URL")?;

    let id_token = cognito_env::get_token_cache_or_auth(
        "ap-northeast-1",
        &secrets_manager_id,
        &[
            "COGNITO_CLIENT_SECRET",
            "COGNITO_CLIENT_ID",
            "COGNITO_USER_POOL_ID",
        ],
        &username,
        &password,
    )
    .await?
    .0
    .as_str();

    let reqwest_client = reqwest::Client::new();

    let lambda_url = api_gateway_url;
    let response = reqwest_client
        .post(lambda_url)
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await?;

    let body = response.json::<User>().await?;

    tracing::warn!("{:?}", body);

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    pub id: String,
    pub username: String,
}
