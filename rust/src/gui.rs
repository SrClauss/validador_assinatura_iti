use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod api;
mod types;

use api::{download_relatorio_pdf, get_conformidade_report, validate_pdf};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Validador ITI - Assinaturas Digitais",
        options,
        Box::new(|_cc| Ok(Box::new(ValidadorApp::new()))),
    )
}

struct ValidadorApp {
    pdf_path: Option<PathBuf>,
    language: Language,
    resultado: String,
    status_message: String,
    processing: bool,
    gerar_relatorio: bool,
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl Default for ValidadorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidadorApp {
    fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            pdf_path: None,
            language: Language::default(),
            resultado: String::new(),
            status_message: String::new(),
            processing: false,
            gerar_relatorio: false,
            tx,
            rx,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    PtBr,
    En,
    Es,
}

impl Default for Language {
    fn default() -> Self {
        Language::PtBr
    }
}

impl Language {
    fn as_str(&self) -> &str {
        match self {
            Language::PtBr => "pt-br",
            Language::En => "en",
            Language::Es => "es",
        }
    }

    fn display(&self) -> &str {
        match self {
            Language::PtBr => "Português (BR)",
            Language::En => "English",
            Language::Es => "Español",
        }
    }
}

impl ValidadorApp {
    fn validar_pdf(&mut self) {
        if let Some(ref path) = self.pdf_path {
            let path_clone = path.clone();
            let gerar_relatorio = self.gerar_relatorio;
            let language = self.language;
            let tx = self.tx.clone();

            self.processing = true;
            self.status_message = "Validando PDF...".to_string();
            self.resultado.clear();

            thread::spawn(move || {
                let resultado = validate_pdf(&path_clone, false);

                let texto = match resultado {
                    Ok(res) => {
                        let json_pretty = serde_json::to_string_pretty(&res)
                            .unwrap_or_else(|_| "Erro ao formatar JSON".to_string());

                        let mut output = String::new();
                        output.push_str("═══════════════════════════════════════════════\n");
                        output.push_str("  RESULTADO DA VALIDAÇÃO\n");
                        output.push_str("═══════════════════════════════════════════════\n\n");

                        if res.status == "valid" {
                            output.push_str("✓ STATUS: VÁLIDO\n\n");
                            if let Some(total) = res.total_assinaturas {
                                output.push_str(&format!("Total de assinaturas: {}\n\n", total));
                            }

                            if let Some(assinaturas) = &res.assinaturas {
                                for (i, assinatura) in assinaturas.iter().enumerate() {
                                    output.push_str(&format!("Assinatura {}:\n", i + 1));
                                    output.push_str(&format!("  • Assinado por: {}\n", assinatura.assinado_por));
                                    output.push_str(&format!("  • CPF: {}\n", assinatura.cpf));
                                    output.push_str(&format!("  • Certificadora: {}\n", assinatura.certificadora));
                                    output.push_str(&format!("  • Status: {}\n", assinatura.status));
                                    output.push_str(&format!(
                                        "  • Carimbo de tempo: {}\n\n",
                                        if assinatura.possui_carimbo_tempo { "Sim" } else { "Não" }
                                    ));
                                }
                            }
                        } else if res.status == "invalid" {
                            output.push_str("✗ STATUS: INVÁLIDO\n\n");
                            if let Some(error) = &res.error {
                                output.push_str(&format!("Erro: {}\n\n", error));
                            }
                        } else {
                            output.push_str("⚠ STATUS: ERRO\n\n");
                            if let Some(error) = &res.error {
                                output.push_str(&format!("Erro: {}\n\n", error));
                            }
                        }

                        output.push_str("\n───────────────────────────────────────────────\n");
                        output.push_str("  JSON COMPLETO\n");
                        output.push_str("───────────────────────────────────────────────\n\n");
                        output.push_str(&json_pretty);

                        // Gerar relatório se solicitado
                        if gerar_relatorio && res.status == "valid" {
                            output.push_str("\n\n═══════════════════════════════════════════════\n");
                            output.push_str("  GERANDO RELATÓRIO PDF\n");
                            output.push_str("═══════════════════════════════════════════════\n\n");

                            match get_conformidade_report(&path_clone, false) {
                                Ok(relatorio_result) => {
                                    if relatorio_result.status == "success" {
                                        if let Some(relatorio) = relatorio_result.relatorio_conformidade {
                                            let output_path = path_clone
                                                .parent()
                                                .unwrap_or(std::path::Path::new("."))
                                                .join(format!(
                                                    "Relatorio_{}",
                                                    path_clone.file_name().unwrap().to_string_lossy()
                                                ));

                                            match download_relatorio_pdf(
                                                &relatorio,
                                                language.as_str(),
                                                Some(&output_path),
                                                false,
                                            ) {
                                                Ok(pdf_result) => {
                                                    if pdf_result.status == "success" {
                                                        output.push_str(&format!(
                                                            "✓ Relatório PDF salvo em:\n  {}\n",
                                                            output_path.display()
                                                        ));
                                                    } else {
                                                        output.push_str(&format!(
                                                            "✗ Erro ao baixar PDF: {}\n",
                                                            pdf_result.error.unwrap_or_default()
                                                        ));
                                                    }
                                                }
                                                Err(e) => {
                                                    output.push_str(&format!("✗ Erro ao baixar PDF: {}\n", e));
                                                }
                                            }
                                        }
                                    } else {
                                        output.push_str(&format!(
                                            "✗ Erro ao obter relatório: {}\n",
                                            relatorio_result.error.unwrap_or_default()
                                        ));
                                    }
                                }
                                Err(e) => {
                                    output.push_str(&format!("✗ Erro ao obter relatório: {}\n", e));
                                }
                            }
                        }

                        output
                    }
                    Err(e) => {
                        format!("✗ ERRO\n\n{}", e)
                    }
                };

                // Send result back to main thread
                let _ = tx.send(texto);
            });
        }
    }
}

impl eframe::App for ValidadorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for results from background thread
        if let Ok(resultado) = self.rx.try_recv() {
            self.resultado = resultado;
            self.processing = false;
            self.status_message = "Validação concluída!".to_string();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔐 Validador de Assinaturas Digitais - ITI");
            ui.add_space(10.0);

            // Seleção de arquivo
            ui.horizontal(|ui| {
                if ui.button("📁 Selecionar PDF").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PDF", &["pdf"])
                        .pick_file()
                    {
                        self.pdf_path = Some(path);
                        self.resultado.clear();
                        self.status_message.clear();
                    }
                }

                if let Some(ref path) = self.pdf_path {
                    ui.label(format!("Arquivo: {}", path.file_name().unwrap().to_string_lossy()));
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Opções
            ui.horizontal(|ui| {
                ui.label("Idioma do relatório:");
                egui::ComboBox::from_label("")
                    .selected_text(self.language.display())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.language, Language::PtBr, "Português (BR)");
                        ui.selectable_value(&mut self.language, Language::En, "English");
                        ui.selectable_value(&mut self.language, Language::Es, "Español");
                    });
            });

            ui.add_space(5.0);

            ui.checkbox(&mut self.gerar_relatorio, "Gerar relatório PDF após validação");

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Botões de ação
            ui.horizontal(|ui| {
                let can_validate = self.pdf_path.is_some() && !self.processing;

                if ui
                    .add_enabled(can_validate, egui::Button::new("✓ Validar"))
                    .clicked()
                {
                    self.validar_pdf();
                }

                if ui.button("🗑 Limpar").clicked() {
                    self.pdf_path = None;
                    self.resultado.clear();
                    self.status_message.clear();
                }
            });

            ui.add_space(10.0);

            // Status
            if !self.status_message.is_empty() {
                ui.label(&self.status_message);
                ui.add_space(5.0);
            }

            // Área de resultado
            ui.separator();
            ui.add_space(5.0);
            ui.label("Resultado:");
            ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.resultado.as_str())
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20),
                    );
                });
        });

        // Request repaint to keep UI responsive
        ctx.request_repaint();
    }
}
