use aws_sdk_cognitoidentityprovider as provider;
use serde::{Deserialize, Serialize};
use srp::SRP;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ChallengeParameters {
    // SESSION: String, // 独自追加
    SALT: String,
    SRP_B: String,
    SECRET_BLOCK: String,
    USER_ID_FOR_SRP: String,
    USERNAME: String,
}

async fn request_challenge() -> anyhow::Result<ChallengeParameters> {
    tracing::trace!("request_challengeに入りました。");
    eprintln!("request_challengeに入りました。");

    let url = "https://533sdf18m7.execute-api.ap-northeast-1.amazonaws.com/dev/auth";
    let response = reqwest::get(url).await?;
    tracing::trace!("reqwestのGETメソッドでSecondTestのURLにアクセスしました。");
    eprintln!("{:?}", response);

    // 認証情報をパース
    let challenge_parameters_result = response.json().await;
    eprintln!("{:?}", challenge_parameters_result);
    let challenge_parameters = challenge_parameters_result?;
    tracing::trace!("responseをjsonパースしました。");
    // eprintln!("responseをjsonパースしました。");

    Ok(challenge_parameters)
    // unimplemented!()
}

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
    tracing::info!("get_auth_info_from_envを実行しました。");

    let username = "Akira";
    let secret_hash = {
        use aws_get_secret_value::get_secret_hash;
        get_secret_hash(&username, &secret_key, &client_id)?
    };
    tracing::info!("シークレットハッシュを取得しました。");

    let challenge_parameters = request_challenge().await?;
    tracing::info!("challengeを取得しました。");

    let ChallengeParameters {
        // SESSION: session,
        SALT: salt,
        SRP_B: srp_b,
        SECRET_BLOCK: secret_block,
        USER_ID_FOR_SRP: user_id_for_srp,
        // USERNAME: username,
        ..
    } = challenge_parameters.clone();

    let srp_client = SRP::new(user_pool_id);
    tracing::info!("srp_clientを生成しました。");

    let srp_a = srp_client.calculate_A().to_string();
    let password = "".to_string();

    let hkdf = srp_client.get_authenticate_key(
        &user_id_for_srp,
        &password,
        &srp_a,
        &srp_b,
        salt.as_bytes(),
    );
    let (timestamp, signature) = srp_client.sign(&hkdf, &user_id_for_srp, &secret_block);
    tracing::info!("署名を取得しました。");

    use aws_sdk_cognitoidentityprovider::types::ChallengeNameType;
    let respond = client
        .respond_to_auth_challenge()
        .client_id(client_id)
        .challenge_name(ChallengeNameType::PasswordVerifier)
        // .session(session)
        .challenge_responses("TIMESTAMP", timestamp)
        .challenge_responses("USERNAME", user_id_for_srp)
        .challenge_responses("PASSWORD_CLAIM_SECRET_BLOCK", secret_block)
        .challenge_responses("PASSWORD_CLAIM_SIGNATURE", signature)
        .challenge_responses("SECRET_HASH", secret_hash)
        .send()
        .await?;

    tracing::warn!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?}", respond);

    Ok(())
}
