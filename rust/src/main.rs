use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod api;
mod types;

use api::{download_relatorio_pdf, get_conformidade_report, validate_pdf};

#[derive(Parser)]
#[command(name = "validador_iti")]
#[command(author = "Validador ITI Contributors")]
#[command(version = "1.0.0")]
#[command(about = "Validador de assinaturas digitais em PDFs usando API do ITI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Valida assinatura de um PDF
    Validar {
        /// Caminho do arquivo PDF
        #[arg(value_name = "PDF")]
        pdf_path: PathBuf,

        /// Modo verboso
        #[arg(short, long)]
        verbose: bool,
    },

    /// Gera relatório PDF de validação
    GerarRelatorio {
        /// Caminho do arquivo PDF
        #[arg(value_name = "PDF")]
        pdf_path: PathBuf,

        /// Idioma do relatório (pt-br, en, es)
        #[arg(short, long, default_value = "pt-br")]
        language: String,

        /// Caminho de saída do relatório
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Modo verboso
        #[arg(short, long)]
        verbose: bool,
    },

    /// Validar e gerar relatório em um único comando
    Completo {
        /// Caminho do arquivo PDF
        #[arg(value_name = "PDF")]
        pdf_path: PathBuf,

        /// Idioma do relatório (pt-br, en, es)
        #[arg(short, long, default_value = "pt-br")]
        language: String,

        /// Caminho de saída do relatório
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Modo verboso
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validar { pdf_path, verbose } => {
            if verbose {
                println!("\n{}", "=".repeat(60));
                println!("Validando: {}", pdf_path.display());
                println!("{}\n", "=".repeat(60));
            }

            let resultado = validate_pdf(&pdf_path, verbose)
                .context("Erro ao validar PDF")?;

            println!("{}", serde_json::to_string_pretty(&resultado)?);

            if verbose {
                println!("\n{}", "=".repeat(60));
                match resultado.status.as_str() {
                    "valid" => {
                        println!("✓ Status: VÁLIDO");
                        println!("✓ Assinaturas: {}", resultado.total_assinaturas.unwrap_or(0));
                    }
                    "invalid" => println!("✗ Status: INVÁLIDO"),
                    "error" => println!("✗ Status: ERRO"),
                    _ => println!("? Status: DESCONHECIDO"),
                }
                println!("{}\n", "=".repeat(60));
            }
        }

        Commands::GerarRelatorio {
            pdf_path,
            language,
            output,
            verbose,
        } => {
            if !["pt-br", "en", "es"].contains(&language.as_str()) {
                anyhow::bail!("Idioma inválido. Use: pt-br, en ou es");
            }

            if verbose {
                println!("\n{}", "=".repeat(60));
                println!("Gerando relatório: {}", pdf_path.display());
                println!("Idioma: {}", language);
                println!("{}\n", "=".repeat(60));
            }

            let relatorio = get_conformidade_report(&pdf_path, verbose)
                .context("Erro ao obter relatório de conformidade")?;

            if relatorio.status != "success" {
                anyhow::bail!("Falha ao obter relatório: {}", relatorio.error.unwrap_or_default());
            }

            let output_path = output.unwrap_or_else(|| {
                PathBuf::from(format!(
                    "Relatorio_{}",
                    pdf_path.file_name().unwrap().to_string_lossy()
                ))
            });

            let pdf_result = download_relatorio_pdf(
                &relatorio.relatorio_conformidade.unwrap(),
                &language,
                Some(&output_path),
                verbose,
            )
            .context("Erro ao baixar PDF do relatório")?;

            if pdf_result.status != "success" {
                anyhow::bail!("Falha ao baixar PDF: {}", pdf_result.error.unwrap_or_default());
            }

            if verbose {
                println!("\n{}", "=".repeat(60));
                println!("✓ Relatório salvo em: {}", output_path.display());
                println!("{}\n", "=".repeat(60));
            } else {
                println!("Relatório salvo: {}", output_path.display());
            }
        }

        Commands::Completo {
            pdf_path,
            language,
            output,
            verbose,
        } => {
            if !["pt-br", "en", "es"].contains(&language.as_str()) {
                anyhow::bail!("Idioma inválido. Use: pt-br, en ou es");
            }

            if verbose {
                println!("\n{}", "=".repeat(60));
                println!("Validação completa: {}", pdf_path.display());
                println!("{}\n", "=".repeat(60));
            }

            // 1. Validar
            let validacao = validate_pdf(&pdf_path, verbose)
                .context("Erro ao validar PDF")?;

            println!("\n📊 Resultado da Validação:");
            println!("{}\n", serde_json::to_string_pretty(&validacao)?);

            // 2. Obter relatório
            let relatorio = get_conformidade_report(&pdf_path, verbose)
                .context("Erro ao obter relatório de conformidade")?;

            if relatorio.status != "success" {
                anyhow::bail!("Falha ao obter relatório: {}", relatorio.error.unwrap_or_default());
            }

            // 3. Baixar PDF
            let output_path = output.unwrap_or_else(|| {
                PathBuf::from(format!(
                    "Relatorio_{}",
                    pdf_path.file_name().unwrap().to_string_lossy()
                ))
            });

            let pdf_result = download_relatorio_pdf(
                &relatorio.relatorio_conformidade.unwrap(),
                &language,
                Some(&output_path),
                verbose,
            )
            .context("Erro ao baixar PDF do relatório")?;

            if pdf_result.status != "success" {
                anyhow::bail!("Falha ao baixar PDF: {}", pdf_result.error.unwrap_or_default());
            }

            println!("\n{}", "=".repeat(60));
            println!("✓ Validação: {}", validacao.status);
            if let Some(total) = validacao.total_assinaturas {
                println!("✓ Assinaturas: {}", total);
            }
            println!("✓ Relatório: {}", output_path.display());
            println!("{}\n", "=".repeat(60));
        }
    }

    Ok(())
}
