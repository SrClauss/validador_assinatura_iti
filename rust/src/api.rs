use crate::types::*;
use anyhow::{Context, Result};
use reqwest::blocking::{multipart, Client};
use serde_json::Value;
use std::fs;
use std::path::Path;

const URL_ARQUIVO: &str = "https://validar.iti.gov.br/arquivo";
const URL_SIMPLES: &str = "https://validar.iti.gov.br/simples";
const URL_CONFORMIDADE: &str = "https://validar.iti.gov.br/conformidade";
const URL_DOWNLOAD_PDF: &str = "https://validar.iti.gov.br/downloadPdf";

fn create_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("Failed to create HTTP client")
}

fn get_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Referer", "https://validar.iti.gov.br/".parse().unwrap());
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"142\", \"Google Chrome\";v=\"142\", \"Not_A Brand\";v=\"99\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Origin", "https://validar.iti.gov.br".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers
}

pub fn validate_pdf(pdf_path: &Path, verbose: bool) -> Result<ValidationResult> {
    if !pdf_path.exists() {
        return Ok(ValidationResult {
            status: "error".to_string(),
            documento: None,
            assinaturas: None,
            total_assinaturas: None,
            error: Some(format!("Arquivo não encontrado: {}", pdf_path.display())),
            details: None,
        });
    }

    if verbose {
        println!("📤 Enviando PDF para /arquivo...");
    }

    let client = create_client();
    let headers = get_headers();

    // Etapa 1: Upload do arquivo
    let file_name = pdf_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let file_content = fs::read(pdf_path)
        .context("Erro ao ler arquivo PDF")?;

    let form = multipart::Form::new()
        .part(
            "signature_files[]",
            multipart::Part::bytes(file_content)
                .file_name(file_name.clone())
                .mime_str("application/pdf")?,
        );

    let response = client
        .post(URL_ARQUIVO)
        .headers(headers.clone())
        .multipart(form)
        .send()
        .context("Erro ao enviar requisição para /arquivo")?;

    let status_code = response.status();

    if verbose {
        println!("   Status: {}", status_code);
    }

    if status_code == 400 {
        return Ok(ValidationResult {
            status: "invalid".to_string(),
            documento: None,
            assinaturas: None,
            total_assinaturas: None,
            error: Some("Documento sem assinatura ou inválido".to_string()),
            details: response.json().ok(),
        });
    }

    if !status_code.is_success() {
        return Ok(ValidationResult {
            status: "error".to_string(),
            documento: None,
            assinaturas: None,
            total_assinaturas: None,
            error: Some(format!("Erro HTTP {}", status_code)),
            details: Some(serde_json::Value::String(response.text()?)),
        });
    }

    let json_bruto: Value = response.json()
        .context("Erro ao parsear resposta de /arquivo")?;

    if verbose {
        println!("   ✓ Resposta recebida ({} bytes)", serde_json::to_string(&json_bruto)?.len());
    }

    // Etapa 2: Processar com /simples
    if verbose {
        println!("📥 Processando com /simples...");
    }

    let mut headers_simples = get_headers();
    headers_simples.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
    headers_simples.insert("Content-Type", "application/json".parse().unwrap());

    let response_simples = client
        .post(URL_SIMPLES)
        .headers(headers_simples)
        .json(&json_bruto)
        .send()
        .context("Erro ao enviar requisição para /simples")?;

    let status_simples = response_simples.status();

    if verbose {
        println!("   Status: {}", status_simples);
    }

    if !status_simples.is_success() {
        return Ok(ValidationResult {
            status: "error".to_string(),
            documento: None,
            assinaturas: None,
            total_assinaturas: None,
            error: Some(format!("Erro no /simples: {}", status_simples)),
            details: Some(serde_json::Value::String(response_simples.text()?)),
        });
    }

    let relatorio: Value = response_simples.json()
        .context("Erro ao parsear resposta de /simples")?;

    if verbose {
        println!("   ✓ Relatório recebido\n");
    }

    // Processar relatório
    process_relatorio(&relatorio, &file_name)
}

fn process_relatorio(relatorio: &Value, filename: &str) -> Result<ValidationResult> {
    let mut assinaturas = Vec::new();

    if let Some(obj) = relatorio.as_object() {
        let assinaturas_raw = if let Some(assinaturas_val) = obj.get("assinaturas") {
            assinaturas_val.as_array()
        } else if let Some(signatures_val) = obj.get("signatures") {
            signatures_val.as_array()
        } else {
            None
        };

        if let Some(assinaturas_array) = assinaturas_raw {
            for assinatura in assinaturas_array {
                if let Some(assinatura_obj) = assinatura.as_object() {
                    let assinatura_info = Assinatura {
                        assinado_por: assinatura_obj
                            .get("nome")
                            .or(assinatura_obj.get("signerName"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("N/A")
                            .to_string(),
                        cpf: assinatura_obj
                            .get("cpf")
                            .or(assinatura_obj.get("CPF"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("N/A")
                            .to_string(),
                        certificadora: assinatura_obj
                            .get("certificadora")
                            .and_then(|v| v.as_str())
                            .unwrap_or("N/A")
                            .to_string(),
                        numero_serie_certificado: assinatura_obj
                            .get("numSerial")
                            .or(assinatura_obj.get("serialNumber"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("N/A")
                            .to_string(),
                        data_assinatura: assinatura_obj
                            .get("data")
                            .or(assinatura_obj.get("signatureDate"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("N/A")
                            .to_string(),
                        status: assinatura_obj
                            .get("status")
                            .or(assinatura_obj.get("resultado"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("N/A")
                            .to_string(),
                        possui_carimbo_tempo: assinatura_obj
                            .get("possuiCarimboTempo")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    };
                    assinaturas.push(assinatura_info);
                }
            }
        }

        let doc_info = Documento {
            nome_arquivo: obj
                .get("nomeArquivo")
                .and_then(|v| v.as_str())
                .unwrap_or(filename)
                .to_string(),
            hash: obj
                .get("hash")
                .or(obj.get("documentHash"))
                .and_then(|v| v.as_str())
                .unwrap_or("N/A")
                .to_string(),
            data_validacao: obj
                .get("dataValidacao")
                .or(obj.get("validationDate"))
                .and_then(|v| v.as_str())
                .unwrap_or("N/A")
                .to_string(),
            status_documento: obj
                .get("statusDocumento")
                .and_then(|v| v.as_str())
                .unwrap_or("N/A")
                .to_string(),
        };

        Ok(ValidationResult {
            status: if !assinaturas.is_empty() { "valid" } else { "invalid" }.to_string(),
            documento: Some(doc_info),
            assinaturas: Some(assinaturas.clone()),
            total_assinaturas: Some(assinaturas.len()),
            error: None,
            details: None,
        })
    } else {
        Ok(ValidationResult {
            status: "unknown".to_string(),
            documento: None,
            assinaturas: None,
            total_assinaturas: None,
            error: Some("Formato de resposta desconhecido".to_string()),
            details: Some(relatorio.clone()),
        })
    }
}

pub fn get_conformidade_report(pdf_path: &Path, verbose: bool) -> Result<ConformidadeResult> {
    if !pdf_path.exists() {
        return Ok(ConformidadeResult {
            status: "error".to_string(),
            relatorio_conformidade: None,
            json_bruto: None,
            error: Some(format!("Arquivo não encontrado: {}", pdf_path.display())),
            details: None,
        });
    }

    if verbose {
        println!("\n{}", "=".repeat(60));
        println!("Obtendo relatório de conformidade: {}", pdf_path.display());
        println!("{}\n", "=".repeat(60));
    }

    let client = create_client();
    let headers = get_headers();

    // Etapa 1: Upload do arquivo
    if verbose {
        println!("📤 Enviando PDF para /arquivo...");
    }

    let file_name = pdf_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let file_content = fs::read(pdf_path)
        .context("Erro ao ler arquivo PDF")?;

    let form = multipart::Form::new()
        .part(
            "signature_files[]",
            multipart::Part::bytes(file_content)
                .file_name(file_name)
                .mime_str("application/pdf")?,
        );

    let response = client
        .post(URL_ARQUIVO)
        .headers(headers.clone())
        .multipart(form)
        .send()
        .context("Erro ao enviar requisição para /arquivo")?;

    let status_code = response.status();

    if verbose {
        println!("   Status: {}", status_code);
    }

    if status_code == 400 {
        return Ok(ConformidadeResult {
            status: "invalid".to_string(),
            relatorio_conformidade: None,
            json_bruto: None,
            error: Some("Documento sem assinatura ou inválido".to_string()),
            details: response.json().ok(),
        });
    }

    if !status_code.is_success() {
        return Ok(ConformidadeResult {
            status: "error".to_string(),
            relatorio_conformidade: None,
            json_bruto: None,
            error: Some(format!("Erro HTTP {} em /arquivo", status_code)),
            details: Some(serde_json::Value::String(response.text()?)),
        });
    }

    let json_bruto: Value = response.json()
        .context("Erro ao parsear resposta de /arquivo")?;

    if verbose {
        println!("   ✓ Resposta recebida ({} bytes)", serde_json::to_string(&json_bruto)?.len());
    }

    // Etapa 2: Obter relatório de conformidade
    if verbose {
        println!("📥 Processando com /conformidade...");
    }

    let mut headers_conformidade = get_headers();
    headers_conformidade.insert("Accept", "application/json".parse().unwrap());
    headers_conformidade.insert("Content-Type", "application/json".parse().unwrap());

    let response_conformidade = client
        .post(URL_CONFORMIDADE)
        .headers(headers_conformidade)
        .json(&json_bruto)
        .send()
        .context("Erro ao enviar requisição para /conformidade")?;

    let status_conformidade = response_conformidade.status();

    if verbose {
        println!("   Status: {}", status_conformidade);
    }

    if !status_conformidade.is_success() {
        return Ok(ConformidadeResult {
            status: "error".to_string(),
            relatorio_conformidade: None,
            json_bruto: Some(json_bruto),
            error: Some(format!("Erro HTTP {} em /conformidade", status_conformidade)),
            details: Some(serde_json::Value::String(response_conformidade.text()?)),
        });
    }

    let relatorio_conformidade: Value = response_conformidade.json()
        .context("Erro ao parsear resposta de /conformidade")?;

    if verbose {
        println!("   ✓ Relatório de conformidade recebido\n");
        println!("{}\n", "=".repeat(60));
    }

    Ok(ConformidadeResult {
        status: "success".to_string(),
        relatorio_conformidade: Some(relatorio_conformidade),
        json_bruto: Some(json_bruto),
        error: None,
        details: None,
    })
}

pub fn download_relatorio_pdf(
    relatorio_conformidade: &Value,
    language: &str,
    save_as: Option<&Path>,
    verbose: bool,
) -> Result<PdfDownloadResult> {
    if !["pt-br", "en", "es"].contains(&language) {
        return Ok(PdfDownloadResult {
            status: "error".to_string(),
            pdf_path: None,
            pdf_size: None,
            error: Some(format!("Idioma inválido: {}. Use 'pt-br', 'en' ou 'es'", language)),
            details: None,
        });
    }

    if verbose {
        println!("\n{}", "=".repeat(60));
        println!("Download do PDF do relatório (idioma: {})", language);
        println!("{}\n", "=".repeat(60));
    }

    let client = create_client();
    let mut headers = get_headers();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    if verbose {
        println!("📥 Baixando PDF do relatório...");
    }

    let body = serde_json::json!({
        "data": serde_json::to_string(relatorio_conformidade)?,
        "language": language
    });

    let response = client
        .post(URL_DOWNLOAD_PDF)
        .headers(headers)
        .json(&body)
        .send()
        .context("Erro ao enviar requisição para /downloadPdf")?;

    let status_code = response.status();

    if verbose {
        println!("   Status: {}", status_code);
    }

    if !status_code.is_success() {
        return Ok(PdfDownloadResult {
            status: "error".to_string(),
            pdf_path: None,
            pdf_size: None,
            error: Some(format!("Erro HTTP {} em /downloadPdf", status_code)),
            details: Some(response.text()?),
        });
    }

    let pdf_bytes = response.bytes()
        .context("Erro ao ler bytes do PDF")?;

    let pdf_size = pdf_bytes.len();

    if verbose {
        println!("   ✓ PDF recebido ({} bytes)", pdf_size);
    }

    let pdf_path = if let Some(path) = save_as {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Erro ao criar diretórios")?;
        }
        fs::write(path, &pdf_bytes)
            .context("Erro ao salvar PDF")?;

        if verbose {
            println!("   ✓ Salvo em: {}", path.display());
        }

        Some(path.to_string_lossy().to_string())
    } else {
        None
    };

    if verbose {
        println!("{}\n", "=".repeat(60));
    }

    Ok(PdfDownloadResult {
        status: "success".to_string(),
        pdf_path,
        pdf_size: Some(pdf_size),
        error: None,
        details: None,
    })
}
