use chrono::Local;
use serde::Serialize;
use std::io::Write;
#[derive(Serialize)]
struct Etapa1 {
    data: String,
    acido_salicilico: f64,
    anidrido_acetico: f64,
    acido_fosforico: f64,
    aspirina_bruta: f64,
    sobra_acido_acetico: f64,
    sobra_anidrido_acetico: f64,
}
fn entrada_f64(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    loop {
        print!("{}", prompt);
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        match buffer.trim().parse::<f64>() {
            Ok(valor) if valor > 0.0 => {
                println!(" ");
                return Ok(valor);
            }
            Ok(_) => println!("Valor deve ser maior que zero. Tente Novamente. "),
            Err(_) => {
                println!("Entrada inválida. Tente novamente:");
                continue;
            }
        }
    }
}
fn secao(titulo: &str) {
    println!(" ");
    println!("======================================================");
    println!("              {}", titulo);
    println!("======================================================");
    println!(" ");
}
fn campo(label: &str, valor: f64) {
    println!("{} {} Kg", label, valor);
    println!(" ");
}

fn campo_string(label: &str, valor: &str) {
    println!("{} {}", label, valor);
    println!(" ");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Local::now().format("%Y-%m-%d").to_string();
    secao("Materias primas recebidas.");
    let materia_prima1 = entrada_f64("Adicione a quantidade de Ácido salicílico recebida em Kg: ")?;
    let materia_prima2 = entrada_f64("Adicione a quantidade de Anidrido acético recebida Kg: ")?;
    let materia_prima3 = entrada_f64("Adicione a quantidade de Ácido fosfórico recebida Kg: ")?;
    secao("Produtos produzidos.");
    let produto = entrada_f64("Adicione a quantidade de Aspirina bruta produzida em Kg: ")?;
    secao("Sobras da reação.");
    let sobra1 = entrada_f64("Adicione a quantidade de sobra de Ácido acético produzida em Kg: ")?;
    let sobra2 = entrada_f64("Adicione a quantidade de sobra de Anidrido acético em Kg: ")?;
    secao("Confirmação de envio");
    secao("Data de envio.");
    campo_string("O arquivo foi enviado a pasta em: ", &data);
    secao("Materias primas recebidas.");
    campo("Ácido salicílico:", materia_prima1);
    campo("Anidrido acético:", materia_prima2);
    campo("Ácido fosfórico:", materia_prima3);
    secao("Produtos produzidos");
    campo("Aspirina bruta:", produto);
    secao("Sobras da reação");
    campo("Ácido acético:", sobra1);
    campo("Anidrido acético:", sobra2);
    secao("Fim da lista.");
    let nome_do_arquivo = format!("etapa1_{}.json", &data);
    let etapa1 = Etapa1 {
        data,
        acido_salicilico: materia_prima1,
        anidrido_acetico: materia_prima2,
        acido_fosforico: materia_prima3,
        aspirina_bruta: produto,
        sobra_acido_acetico: sobra1,
        sobra_anidrido_acetico: sobra2,
    };
    let json = serde_json::to_string_pretty(&etapa1)?;
    std::fs::write(&nome_do_arquivo, json)?;
    println!("Dados salvos {}", nome_do_arquivo);
    Ok(())
}
