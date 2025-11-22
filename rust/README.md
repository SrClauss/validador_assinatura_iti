# Validador ITI - Versão Rust

Implementação em Rust do validador de assinaturas digitais em PDFs usando a API do ITI.

## 🖥️ Interface Gráfica (GUI)

**Executável com interface visual moderna!**

```bash
# Executar interface gráfica
./bin/validador_iti_gui
```

**Funcionalidades da GUI:**
- 📁 Seletor de arquivos PDF visual
- ✓ Botão de validação com feedback em tempo real
- 🌍 Seleção de idioma do relatório (Português, English, Español)
- 📄 Geração automática de relatório PDF
- 📊 Visualização detalhada dos resultados
- 🎨 Interface moderna usando egui

**Tamanho:** 8.7 MB (executável standalone)

![Screenshot da GUI - Interface moderna e intuitiva]

---

## 🚀 CLI (Linha de Comando)

Um executável CLI também está disponível em:
```
../bin/validador_iti
```

### Uso Rápido

```bash
# Validar PDF
./bin/validador_iti validar documento.pdf

# Gerar relatório PDF
./bin/validador_iti gerar-relatorio documento.pdf -o relatorio.pdf

# Validação completa (validar + gerar relatório)
./bin/validador_iti completo documento.pdf -l pt-br -o relatorio.pdf
```

## 📦 Compilação

### Pré-requisitos

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compilar

```bash
cd rust

# Compilar CLI
cargo build --release --bin validador_iti

# Compilar GUI
cargo build --release --bin validador_iti_gui

# Compilar ambos
cargo build --release
```

Os executáveis serão gerados em:
- CLI: `target/release/validador_iti`
- GUI: `target/release/validador_iti_gui`

### Compilação Otimizada

O projeto já está configurado para compilação otimizada em `Cargo.toml`:
- `opt-level = "z"` - Tamanho mínimo
- `lto = true` - Link-Time Optimization
- `codegen-units = 1` - Otimização máxima
- `strip = true` - Remove símbolos de debug

## 💻 Comandos Disponíveis

### 1. `validar`

Valida assinatura de um PDF e retorna JSON com informações.

```bash
# Validação simples
validador_iti validar documento.pdf

# Modo verboso
validador_iti validar documento.pdf -v
```

**Saída:**
```json
{
  "status": "valid",
  "documento": {
    "nome_arquivo": "documento.pdf",
    "hash": "abc123...",
    "data_validacao": "2025-11-22T14:30:00Z",
    "status_documento": "válido"
  },
  "assinaturas": [
    {
      "assinado_por": "João Silva Santos",
      "cpf": "123.456.789-00",
      "certificadora": "AC Raiz Brasileira v5",
      "numero_serie_certificado": "ABC123",
      "data_assinatura": "2025-11-20T10:00:00Z",
      "status": "válida",
      "possui_carimbo_tempo": true
    }
  ],
  "total_assinaturas": 1
}
```

### 2. `gerar-relatorio`

Gera o PDF do relatório de validação do ITI.

```bash
# Relatório em português (padrão)
validador_iti gerar-relatorio documento.pdf

# Especificar idioma e saída
validador_iti gerar-relatorio documento.pdf -l en -o report.pdf

# Modo verboso
validador_iti gerar-relatorio documento.pdf -v
```

**Opções:**
- `-l, --language <LANGUAGE>` - Idioma: `pt-br`, `en`, `es` (padrão: `pt-br`)
- `-o, --output <OUTPUT>` - Caminho de saída (padrão: `Relatorio_<nome>.pdf`)
- `-v, --verbose` - Modo verboso

### 3. `completo`

Executa validação e gera relatório em um único comando.

```bash
# Validação completa
validador_iti completo documento.pdf

# Com todas as opções
validador_iti completo documento.pdf -l pt-br -o relatorio.pdf -v
```

## 📊 Exemplos Práticos

### Validação Simples

```bash
$ validador_iti validar contrato_assinado.pdf
{
  "status": "valid",
  "total_assinaturas": 2,
  ...
}
```

### Pipeline com jq

```bash
# Extrair apenas o status
validador_iti validar doc.pdf | jq -r '.status'

# Listar signatários
validador_iti validar doc.pdf | jq -r '.assinaturas[].assinado_por'

# Contar assinaturas válidas
validador_iti validar doc.pdf | jq '.total_assinaturas'
```

### Script Bash

```bash
#!/bin/bash
for pdf in *.pdf; do
    echo "Validando: $pdf"
    validador_iti validar "$pdf" | jq -r '.status'
done
```

### Validação em Massa

```bash
# Validar todos os PDFs em um diretório
find /path/to/pdfs -name "*.pdf" -exec validador_iti validar {} \;

# Gerar relatórios para todos
for pdf in *.pdf; do
    validador_iti gerar-relatorio "$pdf" -o "relatorio_${pdf}"
done
```

## 🔧 Desenvolvimento

### Estrutura do Projeto

```
rust/
├── src/
│   ├── main.rs       # CLI e entrada principal
│   ├── api.rs        # Implementação das APIs do ITI
│   └── types.rs      # Estruturas de dados
├── Cargo.toml        # Configuração do projeto
└── README.md         # Este arquivo
```

### Dependências

- `reqwest` - Cliente HTTP com suporte a multipart
- `serde` / `serde_json` - Serialização JSON
- `clap` - Interface de linha de comando
- `tokio` - Runtime assíncrono
- `anyhow` - Tratamento de erros

### Executar em Modo Debug

```bash
cargo run -- validar documento.pdf
cargo run -- gerar-relatorio documento.pdf -v
```

### Testes

```bash
cargo test
cargo test -- --nocapture  # Com output
```

### Documentação

```bash
cargo doc --open
```

## 🎯 Vantagens da Versão Rust

1. **Performance** - Executável nativo compilado, muito mais rápido que Python
2. **Tamanho** - Binário único de ~2MB, sem dependências externas
3. **Portabilidade** - Executável standalone, não precisa de runtime
4. **Segurança** - Type-safety em tempo de compilação
5. **Concorrência** - Suporte nativo a async/await

## 🔄 Comparação com Python

| Característica | Rust | Python |
|----------------|------|--------|
| Startup time | ~10ms | ~100ms |
| Memória | ~5MB | ~50MB |
| Portabilidade | Binário único | Requer Python 3.x |
| Performance | Nativa | Interpretada |
| Distribuição | Copiar executável | pip install + deps |

## ⚙️ Opções de Compilação

### Compilação para outros sistemas

```bash
# Linux ARM64
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Windows
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# macOS
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

### Compilação estática (musl)

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## 📝 Licença

MIT License - mesmo da versão Python.

---

**Nota**: Esta é uma implementação independente em Rust da versão Python, mantendo 100% de compatibilidade com a API do ITI.
