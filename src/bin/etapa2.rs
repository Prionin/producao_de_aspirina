use chrono::Local;
use serde::Serialize;
use std::io::Write;
#[derive(Serialize)]
struct Etapa2 {
    data: String,
    aspirina_bruta: f64,
    etanol: f64,
    agua_destilada: f64,
    aspirina_pura: f64,
    solucao_mae: f64,
    etanol_recuperado: f64,
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
    let materia_prima1 = entrada_f64("Adicione a quantidade de Aspirina bruta recebida em Kg: ")?;
    let materia_prima2 = entrada_f64("Adicione a quantidade de Etanol recebida em Kg: ")?;
    let materia_prima3 = entrada_f64("Adicione a quantidade de Água destilada recebida em Kg: ")?;
    secao("Produtos produzidos.");
    let produto = entrada_f64("Adicione a quantidade de Aspirina pura produzida em Kg: ")?;
    secao("Sobras da reação.");
    let sobra1 = entrada_f64("Adicione a quantidade de sobra de Solução mãe produzida em Kg: ")?;
    let sobra2 = entrada_f64("Adicione a quantidade de Etanol recuperado em Kg: ")?;
    secao("Confirmação de envio.");
    secao("Data de envio.");
    campo_string("Enviado a pasta em:", &data);
    secao("Materias primas recebidas.");
    campo("Aspirina bruta:", materia_prima1);
    campo("Etanol:", materia_prima2);
    campo("Água destilada:", materia_prima3);
    secao("Produtos produzidos.");
    campo("Aspirina pura:", produto);
    secao("Sobras da reação.");
    campo("Solução mãe:", sobra1);
    campo("Etanol recuperado:", sobra2);
    let nome_do_arquivo = format!("etapa2_{}.json", &data);
    let etapa2 = Etapa2 {
        data,
        aspirina_bruta: materia_prima1,
        etanol: materia_prima2,
        agua_destilada: materia_prima3,
        aspirina_pura: produto,
        solucao_mae: sobra1,
        etanol_recuperado: sobra2,
    };
    let json = serde_json::to_string_pretty(&etapa2)?;
    std::fs::write(&nome_do_arquivo, json)?;
    println!("Dados salvos em {}", nome_do_arquivo);
    Ok(())
}
