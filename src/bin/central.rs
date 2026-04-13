use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
#[derive(Deserialize)]
struct Etapa2 {
    data: String,
    aspirina_bruta: f64,
    etanol: f64,
    agua_destilada: f64,
    aspirina_pura: f64,
    solucao_mae: f64,
    etanol_recuperado: f64,
}
#[derive(Deserialize)]
struct Etapa1 {
    data: String,
    acido_salicilico: f64,
    anidrido_acetico: f64,
    acido_fosforico: f64,
    aspirina_bruta: f64,
    sobra_acido_acetico: f64,
    sobra_anidrido_acetico: f64,
}
#[derive(Serialize, Deserialize)]
struct Config {
    caminho: String,
}
fn entrada_data(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    loop {
        print!("{}", prompt);
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let data = buffer.trim().to_string();
        match NaiveDate::parse_from_str(&data, "%Y-%m-%d") {
            Ok(_) => {
                println!(" ");
                return Ok(data);
            }
            Err(_) => println!("Data inválida. Use o formato YYYY-MM-DD. Ex: 2026-04-11"),
        }
    }
}
fn entrada_string(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim().to_string();
    Ok(buffer)
}
fn secao(titulo: &str) {
    println!(" ");
    println!("======================================================");
    println!("              {}", titulo);
    println!("======================================================");
    println!(" ");
}
fn campo(label: &str, valor: f64, unidade: &str) {
    println!("{} {:.2} {}", label, valor, unidade);
    println!(" ");
}
fn campo_string(label: &str, valor: String) {
    println!("{} {}", label, valor);
    println!(" ");
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caminho_salvamento: String;
    let home = std::env::var("HOME").unwrap_or_default();
    let config_dir = format!("{}/.config/produção_de_aspirina", home);
    std::fs::create_dir_all(&config_dir)?;
    let config_path = format!("{}/.config/produção_de_aspirina/config.json", home);
    if Path::new(&config_path).exists() {
        let conteudo = std::fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&conteudo)?;
        caminho_salvamento = config.caminho;
    } else {
        let entrada_caminho =
            entrada_string("Adicione o caminho onde estão os arquivos a serem lidos: ")?;
        std::fs::create_dir_all(&entrada_caminho)?;
        let config = Config {
            caminho: entrada_caminho,
        };
        let json = serde_json::to_string_pretty(&config)?;
        std::fs::write(&config_path, json)?;
        caminho_salvamento = config.caminho;
    }
    loop {
        let entrada =
            entrada_data("Adicione a data dos arquivos que quer analisar em formato YYYY-MM-DD: ")?;
        let arquivo1 = format!("{}/etapa1_{}.json", caminho_salvamento, entrada);
        let arquivo2 = format!("{}/etapa2_{}.json", caminho_salvamento, entrada);
        if !std::path::Path::new(&arquivo1).exists() {
            println!("Arquivo de etapa 1 não encontrado para essa data");
            continue;
        }
        if !std::path::Path::new(&arquivo2).exists() {
            println!("Arquivo de etapa 2 não encontrado para essa data");
            continue;
        }

        let conteudo1 = std::fs::read_to_string(arquivo1)?;
        let etapa1: Etapa1 = serde_json::from_str(&conteudo1)?;

        let conteudo2 = std::fs::read_to_string(arquivo2)?;
        let etapa2: Etapa2 = serde_json::from_str(&conteudo2)?;
        let rendimento2 = etapa2.aspirina_pura
            / (etapa2.aspirina_bruta + etapa2.etanol + etapa2.agua_destilada)
            * 100.0;
        let rendimento1 = etapa1.aspirina_bruta
            / (etapa1.acido_salicilico + etapa1.anidrido_acetico + etapa1.acido_fosforico)
            * 100.0;
        let diferenca = (etapa1.aspirina_bruta - etapa2.aspirina_bruta).abs();
        secao("Data dos arquivos analisados.");
        campo_string("A data da etapa 1 analisada é: ", etapa1.data);
        campo_string("A data da etapa 2 analisada é: ", etapa2.data);
        secao("Produção e consumo de Aspirina bruta.");
        campo(
            "A primeira etapa produziu de aspirina bruta:",
            etapa1.aspirina_bruta,
            "Kg",
        );
        campo(
            "A segunda etapa consumiu de aspirina bruta:",
            etapa2.aspirina_bruta,
            "Kg",
        );
        campo(
            "A diferença entre produção e consumo é de:",
            diferenca,
            "Kg",
        );
        secao("Rendimento das etapas.");
        campo("O rendimento da primeira etapa é de:", rendimento1, "%");
        campo("O rendimento da segunda etapa é de:", rendimento2, "%");
        secao("Sobras da primeira etapa.");
        campo(
            "A sobra de ácido acético é de:",
            etapa1.sobra_acido_acetico,
            "Kg",
        );
        campo(
            "A sobra de anidrido acético é de:",
            etapa1.sobra_anidrido_acetico,
            "Kg",
        );
        secao("Sobras da segunda etapa.");
        campo("A sobra de solução mãe é de:", etapa2.solucao_mae, "Kg");

        campo(
            "A sobra de etanol recuperado é de:",
            etapa2.etanol_recuperado,
            "Kg",
        );
        secao("Fim da lista.");
        break;
    }
    Ok(())
}
