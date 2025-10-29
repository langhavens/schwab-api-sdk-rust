use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReceivedCode {
    pub code: oauth2::AuthorizationCode,
    pub state: oauth2::State,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub assertType: String,
}

pub struct SchwabApiClient {
    base_url: String,
    reqwest_client: reqwest::Client,
}

impl SchwabApiClient {
    pub fn new(base_url: String) -> Self {
        SchwabApiClient {
            base_url,
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub async fn get_instruments(&self, cusip: &str) -> Result<Instrument, reqwest::Error> {
        let url = format!("{}/instruments/{}", self.base_url, cusip);
        let response = self.reqwest_client.get(&url).send().await?;
        response.json::<Instrument>().await
    }
}

pub async fn get_oauth2_auth_url() -> Result<oauth2::Url, Box<dyn std::error::Error>> {
    let auth_url = oauth2::Url::parse("")?;
    let token_url = oauth2::Url::parse("")?;
    let redirect_url = oauth2::Url::parse("")?;

    let client_secret = "";

    let mut client = oauth2::Client::new("0", auth_url, token_url);
    client.set_client_secret(client_secret);
    client.set_redirect_url(redirect_url);

    let state = oauth2::State::new_random();

    Ok(client.authorize_url(&state))
}
