use std::fs;
use std::io::{self, Read, Write};
use std::time::Duration;

use cliclack::{intro, log, progress_bar, spinner};
use color_eyre::eyre::Ok;
use console::style;
use serde::Deserialize;
use tokio::time::sleep;

use crate::estacio_client::EstacioClient;
use crate::types::Data;

mod api_client;
mod estacio_client;
mod types;

#[derive(Deserialize)]
struct Config {
    token: String,
}

fn pause() {
    println!("\nPressione Enter para continuar...");
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // Flush stdout to ensure the message is displayed
    stdout.flush().unwrap();

    // Read a single byte (waiting for Enter key)
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let result = run().await;

    if let Err(ref e) = result {
        eprintln!("Erro: {:?}", e);
    }
    pause();

    result
}

async fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;

    intro(style(" Sistema de Marcar como Lido ").on_yellow().black())?;

    // Criar arquivo credenciais.toml se não existir
    if !fs::metadata("credenciais.toml").is_ok() {
        let toml_content = r#"token = "seu_token_aqui""#;
        let mut file = fs::File::create("credenciais.toml").expect("Erro ao criar arquivo");
        file.write_all(toml_content.as_bytes())
            .expect("Erro ao escrever no arquivo");
        log::warning(
            "Arquivo de credenciais criado. Agora, adicione seu bearer token nele antes de prosseguirmos.",
        )?;
        return Ok(());
    }

    log::info("Arquivo de credenciais encontrado.")?;

    // Ler o arquivo credenciais.toml
    let conteudo = fs::read_to_string("credenciais.toml").expect("Erro ao ler arquivo");
    let config: Config = toml::from_str(&conteudo).expect("Erro ao parsear TOML");

    let client = EstacioClient::new(&config.token);
    match client.me().await {
        std::result::Result::Ok(data) => {
            let mut datas: Vec<Data> = Vec::new();
            log::info("Dados do usuário obtidos com sucesso.")?;
            log::info(format!("Autenticado como {}", style(data.nome).yellow()))?;
            log::info(format!(
                "Bearer token {}...",
                &config.token[..config.token.len().min(20)]
            ))?;
            let sp = spinner();
            sp.start("Capturando conteúdos...");
            for matricula in data.matriculas {
                let matricula_id = &matricula.matricula;
                // log::info(format!("Matrícula: {:?}", matricula))?;
                for turma in matricula.turmas {
                    if let std::result::Result::Ok(course) =
                        client.get_course(&turma, matricula_id).await
                    {
                        let course_id = &course.id;
                        log::info(format!("Curso: {}", course_id))?;
                        for tema in course.disciplina.temas {
                            let theme_id = &tema.id;
                            log::info(format!("Tema: {}", theme_id))?;
                            if let std::result::Result::Ok(objetivo) =
                                client.get_theme(&turma, theme_id, matricula_id).await
                            {
                                for conteudo in objetivo.conteudos {
                                    if conteudo.status_conclusao.concluido == true {
                                        continue;
                                    }
                                    let content_id = &conteudo.id;
                                    datas.push(Data {
                                        matricula_id: matricula_id.to_string(),
                                        content_id: content_id.to_string(),
                                        theme_id: theme_id.to_string(),
                                        course_id: course_id.to_string(),
                                    })
                                    // println!("Conteúdo: {}", conteudo.titulo);
                                    // println!("ID do Conteúdo: {}", content_id);
                                    // println!("ID do Tema: {}", theme_id);
                                    // println!("ID do Curso: {}", course_id);
                                }
                            }
                        }
                    }
                }
            }
            let data_size = datas.len() as u64;
            sp.stop(format!(
                "Foram encontrados {} conteúdos para marcar como lidos.",
                data_size
            ));
            let progress = progress_bar(data_size);
            progress.start("Marcando como lido...");
            for data in datas {
                client
                    .post_concluido(&data)
                    .await
                    .expect("Erro ao marcar conteúdo como lido");
                progress.inc(1);
                sleep(Duration::from_secs(1)).await;
            }
            progress.stop(format!(
                "Todos os {} conteúdos foram marcados como lidos.",
                data_size
            ));
        }
        std::result::Result::Err(e) => {
            println!("Erro ao obter dados do usuário: {:?}", e);
        }
    }

    Ok(())
}
