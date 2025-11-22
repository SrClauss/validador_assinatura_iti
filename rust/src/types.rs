use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documento: Option<Documento>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assinaturas: Option<Vec<Assinatura>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_assinaturas: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documento {
    pub nome_arquivo: String,
    pub hash: String,
    pub data_validacao: String,
    pub status_documento: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assinatura {
    pub assinado_por: String,
    pub cpf: String,
    pub certificadora: String,
    pub numero_serie_certificado: String,
    pub data_assinatura: String,
    pub status: String,
    pub possui_carimbo_tempo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConformidadeResult {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relatorio_conformidade: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_bruto: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfDownloadResult {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
