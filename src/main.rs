use std::env;

fn tentar_atualizar() -> Result<(), Box<dyn std::error::Error>> {
    println!("Verificando atualizações...");

    // O env!("CARGO_PKG_VERSION") pega a versão direto do seu Cargo.toml
    let versao_atual = env!("CARGO_PKG_VERSION");

    // Configurando de onde baixar o update (Exemplo usando GitHub)
    let status = self_update::backends::github::Update::configure()
        .repo_owner("aarch6660") // <-- Mude aqui
        .repo_name("Anny_Repo") // <-- Mude aqui
        .bin_name("anny")                     // O nome do seu executável
        .show_download_progress(true)         // Mostra uma barrinha de download!
        .current_version(versao_atual)
        .build()?
        .update()?;

    if status.updated() {
        println!("Atualizado com sucesso para a versão: {}!", status.version());
    } else {
        println!("Você já está na versão mais recente ({}).", versao_atual);
    }

    Ok(())
}

fn main() {
    // 1. Tenta atualizar logo que o programa abre
    if let Err(e) = tentar_atualizar() {
        eprintln!("Erro ao tentar atualizar: {}", e);
        // Não vamos parar o programa se a internet cair, apenas avisamos do erro
    }

    // 2. A lógica principal do seu programa continua aqui em baixo
    println!("---");
    println!("Iniciando o Anny...");
    println!("Olá, mundo! Rodando no Snapdragon!");
}
