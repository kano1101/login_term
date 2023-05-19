use aws_sdk_cognitoidentityprovider as provider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::error!("error message");
    tracing::warn!("warn message");
    tracing::info!("info message");
    tracing::debug!("debug message");
    tracing::trace!("trace message");

    use cognito_secret_hash::get_auth_info_from_env;

    let config = aws_config::load_from_env().await;
    let client = provider::Client::new(&config);

    let (secret_key, client_id, user_pool_id) = get_auth_info_from_env().await?;

    let username = "add5a600-b08c-49fb-ad20-693fd0194eda";
    // let username = "a.kano1101@gmail.com"; これはダメ
    let password = "braQfuqVWtMCNcP6k-2o".to_string();

    let srp_client = cognito_srp::SrpClient::new(
        &username,
        &password,
        &user_pool_id,
        &client_id,
        Some(&secret_key),
    );

    let initiate_auth_response = client
        .initiate_auth()
        .auth_flow(provider::types::AuthFlowType::UserSrpAuth)
        .client_id(client_id.clone())
        .set_auth_parameters(Some(srp_client.get_auth_params().unwrap()))
        .send()
        .await?;

    let challenge_params = initiate_auth_response
        .challenge_parameters
        .ok_or(anyhow::anyhow!("failed to get challenge parameters"))?;

    let challenge_responses = srp_client.process_challenge(challenge_params)?;

    use aws_sdk_cognitoidentityprovider::types::ChallengeNameType;
    let respond = client
        .respond_to_auth_challenge()
        .client_id(client_id)
        .challenge_name(ChallengeNameType::PasswordVerifier)
        .set_challenge_responses(Some(challenge_responses))
        .send()
        .await?;

    let authentication_result = respond.authentication_result.unwrap();
    let id_token = authentication_result.id_token.unwrap();

    tracing::warn!("id_token: {}", id_token);

    Ok(())
}
