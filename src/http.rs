use std::time::Duration;

use anyhow::anyhow;
use reqwest::{Client, Request, RequestBuilder, Response};

pub struct HttpClient {
    req: RequestBuilder,
}

impl HttpClient {
    fn with_defaults(builder: RequestBuilder) -> Self {
        Self {
            req: builder.timeout(Duration::from_secs(30)),
        }
    }

    pub fn get(url: &str) -> Self {
        let builder = Client::new().get(url);

        Self::with_defaults(builder)
    }

    pub fn post(url: &str) -> Self {
        let builder = Client::new().post(url);

        Self::with_defaults(builder)
    }

    pub fn patch(url: &str) -> Self {
        let builder = Client::new().patch(url);

        Self::with_defaults(builder)
    }

    pub fn put(url: &str) -> Self {
        let builder = Client::new().put(url);

        Self::with_defaults(builder)
    }

    pub fn delete(url: &str) -> Self {
        let builder = Client::new().delete(url);

        Self::with_defaults(builder)
    }

    pub async fn send(self) -> Result<(Request, Response), anyhow::Error> {
        let req = self.req.build()?;
        let cloned_req = req
            .try_clone()
            .ok_or(anyhow!("Failed to clone the request"))?;
        let res = Client::new()
            .execute(req)
            .await
            .map_err(|err| anyhow!(err))?;

        Ok((cloned_req, res))
    }

    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.req = self.req.timeout(duration);

        self
    }

    pub fn with_header_from_str(mut self, header: &str) -> Result<Self, anyhow::Error> {
        let header: String = header.chars().filter(|c| !c.is_whitespace()).collect();
        let header: Vec<_> = header.split(':').collect();

        let (k, v) = match (header.first(), header.get(1)) {
            (Some(k), Some(v)) => (k.to_string(), v.to_string()),
            _ => return Err(anyhow!("Invalid header format, must be 'KEY: VALUE'")),
        };

        self.req = self.req.header(k, v);

        Ok(self)
    }

    pub fn with_bearer(mut self, token: &str) -> Self {
        self.req = self.req.bearer_auth(token);

        self
    }

    pub fn with_basic_auth(mut self, credential: &str) -> Result<Self, anyhow::Error> {
        let credential: String = credential.chars().filter(|c| !c.is_whitespace()).collect();
        let credential: Vec<_> = credential.split(':').collect();

        let (user, pass) = match (credential.first(), credential.get(1)) {
            (Some(user), Some(pass)) => (user.to_string(), Some(pass.to_string())),
            (Some(user), None) => (user.to_string(), None),
            _ => {
                return Err(anyhow!(
                    "Invalid basic auth credentials, format must be 'user:password'"
                ))
            }
        };

        self.req = self.req.basic_auth(user, pass);

        Ok(self)
    }

    pub fn with_json_body(mut self, body: String) -> Self {
        self.req = self.req.body(body);
        self.req = self.req.header("Content-Type", "application/json");

        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.req = self.req.body(body);
        self.req = self
            .req
            .header("Content-Type", "application/x-www-form-urlencoded");

        self
    }
}
