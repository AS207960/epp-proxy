#[derive(Copy, Clone)]
pub enum AuthMethod {
    OAuth,
    StaticKey,
}

impl clap::ValueEnum for AuthMethod {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::OAuth, Self::StaticKey]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::OAuth => clap::builder::PossibleValue::new("oauth"),
            Self::StaticKey => clap::builder::PossibleValue::new("static"),
        })
    }
}


#[tonic::async_trait]
pub trait Auth {
    async fn auth(&self, token: &str) -> bool;
}

#[tonic::async_trait]
impl Auth for rust_keycloak::oauth::OAuthClient {
    async fn auth(&self, token: &str) -> bool {
        self.verify_token(token, "access-epp").await.is_ok()
    }
}

#[derive(Clone)]
struct StaticAuth {
    token: String,
}

impl StaticAuth {
    fn new() -> Self {
        dotenv::dotenv().ok();

        let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set");

        Self {
            token: token.trim().to_string(),
        }
    }
}

#[tonic::async_trait]
impl Auth for StaticAuth {
    async fn auth(&self, token: &str) -> bool {
        token == self.token
    }
}

pub fn create_auth_method(method: AuthMethod) -> Box<dyn Auth + Send + Sync> {
    match method {
        AuthMethod::OAuth => Box::new(crate::oauth_client()),
        AuthMethod::StaticKey => Box::new(StaticAuth::new()),
    }
}