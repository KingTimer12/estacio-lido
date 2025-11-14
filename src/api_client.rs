use reqwest::{
    Client, Url,
    header::{HeaderMap, HeaderValue},
};
use std::error::Error;

#[derive(Clone)]
pub struct ApiClient {
    base_url: Url,
    client: Client,
    headers: HeaderMap,
    token: Option<String>,
}

impl ApiClient {
    pub fn new_with_client(base_url: &str, client: Client) -> Result<Self, Box<dyn Error>> {
        let base_url = Url::parse(base_url)?;
        Ok(ApiClient {
            base_url,
            client,
            headers: HeaderMap::new(),
            token: None,
        })
    }

    pub fn update_headers(&mut self, headers: HeaderMap) -> Result<(), Box<dyn Error>> {
        // Merge new headers with existing ones (new ones take precedence)
        for (key, value) in headers.iter() {
            self.headers.insert(key, value.clone());
        }

        // Rebuild the client with all headers
        let builder = Client::builder().default_headers(self.headers.clone());
        self.client = builder.build()?;
        Ok(())
    }

    pub fn set_authorization(&mut self, token: &str) -> Result<(), Box<dyn Error>> {
        // Verificar se há caracteres não-ASCII no token
        if !token.is_ascii() {
            eprintln!("ERRO: Token contém caracteres não-ASCII inválidos!");
            eprintln!("Token recebido: {:?}", token);
            eprintln!("Caracteres problemáticos encontrados:");
            for (i, ch) in token.chars().enumerate() {
                if !ch.is_ascii() {
                    eprintln!("  Posição {}: '{}' (U+{:04X})", i, ch, ch as u32);
                }
            }
            return Err("Token contém caracteres não-ASCII inválidos. Por favor, obtenha um token JWT válido e completo.".into());
        }

        let mut headers = HeaderMap::new();
        let header_value = HeaderValue::from_str(token)?;
        headers.insert("authorization", header_value);
        self.update_headers(headers)?;
        self.token = Some(token.to_string());
        Ok(())
    }
    
    pub async fn get<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        url: &str,
    ) -> Result<T, Box<dyn Error>> {
        let url = self.base_url.join(url)?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let body = response.json().await?;
            return Ok(body);
        }

        let status = response.status();
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        eprintln!("Response body: {}", error_body);
        Err(format!(
            "Request failed with status: {} - Body: {}",
            status, error_body
        )
        .into())
    }

    pub async fn post_without_body(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let url = self.base_url.join(url)?;
        let response = self.client.post(url).send().await?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        eprintln!("Response body: {}", error_body);
        Err(format!(
            "Request failed with status: {} - Body: {}",
            status, error_body
        )
        .into())
    }
}
