#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv(); // Eat error
    let mut args = std::env::args().skip(1);

    let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    std::env::var("TWITCH_OAUTH2_URL")
        .ok()
        .or_else(|| args.next())
        .map(|t| std::env::set_var("TWITCH_OAUTH2_URL", &t))
        .expect("Please set env: TWITCH_OAUTH2_URL or pass url as first argument");

    let client_id = std::env::var("MOCK_CLIENT_ID")
        .ok()
        .or_else(|| args.next())
        .map(twitch_oauth2::ClientId::new)
        .expect("Please set env: MOCK_CLIENT_ID or pass client id as an argument");

    let client_secret = std::env::var("MOCK_CLIENT_SECRET")
        .ok()
        .or_else(|| args.next())
        .map(twitch_oauth2::ClientSecret::new)
        .expect("Please set env: MOCK_CLIENT_SECRET or pass client secret as an argument");

    let user_id = std::env::var("MOCK_USER_ID")
        .ok()
        .or_else(|| args.next())
        .expect("Please set env: MOCK_USER_ID or pass user_id as an argument");

    let token =
        twitch_oauth2::UserToken::mock_token(&reqwest, client_id, client_secret, user_id, vec![])
            .await?;
    println!(
        "token retrieved: {} - {:?}",
        token.access_token.secret(),
        token
    );
    Ok(())
}
