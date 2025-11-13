use std::fs;
use std::io::Write;

use color_eyre::eyre::Ok;
use serde::Deserialize;

use crate::estacio_client::EstacioClient;

mod api_client;
mod estacio_client;
mod types;

#[derive(Deserialize)]
struct Config {
    token: String,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Criar arquivo credenciais.toml se não existir
    if !fs::metadata("credenciais.toml").is_ok() {
        let toml_content = r#"token = "seu_token_aqui""#;
        let mut file = fs::File::create("credenciais.toml").expect("Erro ao criar arquivo");
        file.write_all(toml_content.as_bytes())
            .expect("Erro ao escrever no arquivo");
    }

    // Ler o arquivo credenciais.toml
    let conteudo = fs::read_to_string("credenciais.toml").expect("Erro ao ler arquivo");
    let config: Config = toml::from_str(&conteudo).expect("Erro ao parsear TOML");

    let client = EstacioClient::new(&config.token);
    match client.me().await {
        std::result::Result::Ok(data) => {
            for matricula in data.matriculas {
                for turma in matricula.turmas {
                    if let std::result::Result::Ok(course) =
                        client.get_course(&turma, &matricula.matricula).await
                    {
                        for tema in course.disciplina.temas {
                            if let std::result::Result::Ok(objetivo) = client
                                .get_theme(&turma, &tema.id, &matricula.matricula)
                                .await
                            {
                                for conteudo in objetivo.conteudos {
                                    if conteudo.status_conclusao.concluido == true {
                                        continue;
                                    }
                                    println!("Conteúdo: {}", conteudo.titulo);
                                }
                            }
                        }
                    }
                }
            }
        }
        std::result::Result::Err(e) => {
            println!("Erro ao obter dados do usuário: {:?}", e);
        }
    }

    Ok(())
}
