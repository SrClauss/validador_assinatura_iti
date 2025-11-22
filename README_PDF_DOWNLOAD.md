# Download de PDF do Relatório de Validação ITI

## 🎯 Resumo Rápido

Para baixar o PDF do relatório de validação, você precisa fazer **3 requisições POST**:

```
1. POST /arquivo      → envia PDF, retorna json_bruto
2. POST /conformidade → processa, retorna relatório_conformidade
3. POST /downloadPdf  → gera PDF do relatório
```

## 🚀 Uso Rápido

```bash
# Execute o script de exemplo:
python download_pdf_example.py documento.pdf

# Com idioma específico:
python download_pdf_example.py documento.pdf en
```

## 📋 Endpoint Principal: /downloadPdf

**URL:** `https://validar.iti.gov.br/downloadPdf`
**Método:** `POST`
**Content-Type:** `application/json`

### Headers Necessários:
```http
Content-Type: application/json
Accept: application/json
Referer: https://validar.iti.gov.br/
Origin: https://validar.iti.gov.br
Sec-Fetch-Site: same-origin
Sec-Fetch-Mode: cors
Sec-Fetch-Dest: empty
```

### Body (JSON):
```json
{
  "data": "{\"nomeArquivo\":\"doc.pdf\",\"assinaturas\":[...]}",
  "language": "pt-br"
}
```

**⚠️ IMPORTANTE:** O campo `data` deve ser uma **string JSON** (usar `JSON.stringify()` ou `json.dumps()`)

### Idiomas disponíveis:
- `pt-br` - Português (Brasil)
- `en` - English
- `es` - Español

### Resposta:
Retorna um PDF binário (blob) que pode ser salvo diretamente.

## 📖 Fluxo Completo

### Passo 1: Enviar arquivo para validação
```python
import requests

url = "https://validar.iti.gov.br/arquivo"
files = {'signature_files[]': ('doc.pdf', open('doc.pdf', 'rb'), 'application/pdf')}
headers = {'Referer': 'https://validar.iti.gov.br/'}

response = requests.post(url, files=files, headers=headers)
json_bruto = response.json()
```

### Passo 2: Obter relatório de conformidade
```python
url = "https://validar.iti.gov.br/conformidade"
headers = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    'Referer': 'https://validar.iti.gov.br/'
}

response = requests.post(url, json=json_bruto, headers=headers)
relatorio_conformidade = response.json()
```

### Passo 3: Baixar PDF do relatório
```python
import json

url = "https://validar.iti.gov.br/downloadPdf"
body = {
    "data": json.dumps(relatorio_conformidade),  # ← JSON stringificado!
    "language": "pt-br"
}

response = requests.post(url, json=body, headers=headers)

# Salvar PDF
with open('Relatorio - doc.pdf', 'wb') as f:
    f.write(response.content)
```

## 📁 Arquivos Criados

1. **download_pdf_example.py** - Script completo e funcional
2. **API_ENDPOINTS_DOCUMENTATION.md** - Documentação completa da API
3. **README_PDF_DOWNLOAD.md** - Este arquivo (guia rápido)

## 🔍 Diferença entre /simples e /conformidade

| Endpoint | Uso | Gera PDF? |
|----------|-----|-----------|
| `/simples` | Relatório JSON resumido | ❌ Não |
| `/conformidade` | Relatório JSON completo | ✅ Sim |

**Use `/conformidade`** quando precisar gerar o PDF do relatório.
**Use `/simples`** quando precisar apenas validar programaticamente.

## ⚡ Exemplo cURL

```bash
# 1. Upload
curl -X POST https://validar.iti.gov.br/arquivo \
  -H "Referer: https://validar.iti.gov.br/" \
  -F "signature_files[]=@documento.pdf" \
  -o step1.json

# 2. Conformidade
curl -X POST https://validar.iti.gov.br/conformidade \
  -H "Content-Type: application/json" \
  -H "Referer: https://validar.iti.gov.br/" \
  -d @step1.json \
  -o step2.json

# 3. Download PDF
jq -c '{data: (. | tostring), language: "pt-br"}' step2.json | \
curl -X POST https://validar.iti.gov.br/downloadPdf \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -H "Referer: https://validar.iti.gov.br/" \
  -d @- \
  -o relatorio.pdf
```

## 🐛 Problemas Comuns

### PDF vazio ou corrompido
**Causa:** Campo `data` não foi stringificado
**Solução:** Use `json.dumps(relatorio_conformidade)` em Python ou `JSON.stringify()` em JavaScript

### Erro 400
**Causa:** Documento sem assinatura digital
**Solução:** Verifique se o PDF possui assinatura digital válida

### Erro 422
**Causa:** Documento corrompido
**Solução:** Verifique a integridade do arquivo PDF

## 📚 Documentação Completa

Para detalhes de todos os endpoints, consulte:
- **API_ENDPOINTS_DOCUMENTATION.md** - Documentação completa com todos os endpoints

## 🔗 Links Úteis

- Site oficial: https://validar.iti.gov.br
- Homologação: https://h-validar.iti.gov.br
- Glossário: https://validar.iti.gov.br/glossarioRelatorioDeConformidade.html

---

**Criado em:** 2025-11-22
**Fonte:** Análise do código JavaScript de https://validar.iti.gov.br
