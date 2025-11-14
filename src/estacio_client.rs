use std::error::Error;

use crate::{
    api_client::ApiClient,
    types::{Course, Data, OtherTheme, User},
};

pub struct EstacioClient {
    api: ApiClient,
}

impl EstacioClient {
    pub fn new(token: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Host",
            reqwest::header::HeaderValue::from_static("apis.estudante.estacio.br"),
        );
        headers.insert(
            "Origin",
            reqwest::header::HeaderValue::from_static("https://estudante.estacio.br"),
        );
        headers.insert(
            "Referer",
            reqwest::header::HeaderValue::from_static("https://estudante.estacio.br/"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers.clone())
            .build()
            .expect("Failed to build HTTP client");
        let mut api = ApiClient::new_with_client("https://apis.estudante.estacio.br", client)
            .expect("Failed to build API client");

        // Store the initial headers in the ApiClient
        api.update_headers(headers)
            .expect("Failed to set initial headers");
        api.set_authorization(token)
            .expect("Failed to set authorization");

        Self { api }
    }

    pub async fn me(&self) -> Result<User, Box<dyn Error>> {
        Ok(self.api.get::<User>("/rest/me").await?)
    }

    pub async fn get_course(
        &self,
        course_id: &str,
        matricula_id: &str,
    ) -> Result<Course, Box<dyn Error>> {
        Ok(self
            .api
            .get::<Course>(&format!(
                "/rest/turmas/{}/detalhes?matricula={}",
                course_id, matricula_id
            ))
            .await?)
    }

    pub async fn get_theme(
        &self,
        course_id: &str,
        theme_id: &str,
        matricula_id: &str,
    ) -> Result<OtherTheme, Box<dyn Error>> {
        Ok(self
            .api
            .get::<OtherTheme>(&format!(
                "/rest/turmas/{}/temas/{}?matricula={}",
                course_id, theme_id, matricula_id
            ))
            .await?)
    }

    pub async fn post_concluido(&self, data: &Data) -> Result<(), Box<dyn Error>> {
        Ok(self
            .api
            .post_without_body(&format!(
                "/rest/turmas/{}/temas/{}/conteudos/{}/conclusoes?matricula={}",
                &data.course_id, &data.theme_id, &data.content_id, &data.matricula_id
            ))
            .await?)
    }
}
