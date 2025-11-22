# Documentação da API do Validador ITI

**Base URL:** `https://validar.iti.gov.br`
**Homologação:** `https://h-validar.iti.gov.br`

Esta documentação foi criada através de engenharia reversa do código JavaScript do site oficial.

---

## 📋 Índice

1. [Visão Geral](#visão-geral)
2. [Endpoints Disponíveis](#endpoints-disponíveis)
3. [Fluxo de Validação Simples](#fluxo-de-validação-simples)
4. [Fluxo de Validação com PDF de Relatório](#fluxo-de-validação-com-pdf-de-relatório)
5. [Detalhamento dos Endpoints](#detalhamento-dos-endpoints)
6. [Códigos de Status HTTP](#códigos-de-status-http)
7. [Exemplos de Uso](#exemplos-de-uso)

---

## 🎯 Visão Geral

A API do Validador ITI permite validar assinaturas digitais em documentos PDF, XML e P7S através de requisições HTTP diretas. Existem dois fluxos principais:

- **Fluxo Simples:** Validação + Relatório JSON
- **Fluxo Completo:** Validação + Relatório JSON + Download PDF do Relatório

---

## 🔌 Endpoints Disponíveis

| Endpoint | Método | Descrição |
|----------|--------|-----------|
| `/arquivo` | POST | Envia arquivo para validação |
| `/url` | POST | Valida arquivo a partir de URL |
| `/simples` | POST | Gera relatório simplificado (JSON) |
| `/conformidade` | POST | Gera relatório de conformidade (JSON) |
| `/downloadPdf` | POST | Baixa relatório em formato PDF |
| `/upload` | POST | Envia documento para análise |

---

## 🔄 Fluxo de Validação Simples

```
┌─────────────────┐
│  1. POST /arquivo │
│  (envia PDF)     │
└────────┬─────────┘
         │
         ▼ retorna json_bruto
┌─────────────────┐
│  2. POST /simples│
│  (processa)      │
└────────┬─────────┘
         │
         ▼ retorna relatório JSON
     ✓ Concluído
```

---

## 🔄 Fluxo de Validação com PDF de Relatório

```
┌─────────────────────┐
│  1. POST /arquivo    │
│  (envia PDF)         │
└──────────┬───────────┘
           │
           ▼ retorna json_bruto
┌─────────────────────┐
│  2. POST /conformidade│
│  (processa)          │
└──────────┬───────────┘
           │
           ▼ retorna relatório_conformidade
┌─────────────────────┐
│  3. POST /downloadPdf│
│  (gera PDF)          │
└──────────┬───────────┘
           │
           ▼ retorna PDF blob
      ✓ PDF Baixado
```

---

## 📚 Detalhamento dos Endpoints

### 1. POST /arquivo

**Descrição:** Envia um arquivo PDF para validação de assinatura.

**Headers:**
```http
Content-Type: multipart/form-data
Referer: https://validar.iti.gov.br/
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36
Accept: */*
Origin: https://validar.iti.gov.br
Sec-Fetch-Site: same-origin
Sec-Fetch-Mode: cors
Sec-Fetch-Dest: empty
```

**Body (multipart/form-data):**
```
signature_files[]: <arquivo.pdf>          # Arquivo assinado
detached_files[]: <arquivo_destacado>     # Opcional: arquivo destacado
```

**Resposta (200 OK):**
```json
{
  "identificador": "abc123...",
  "dados_validacao": { ... },
  // Estrutura completa de dados brutos
}
```

**Códigos de Resposta:**
- `200` - Sucesso
- `400` - Documento sem assinatura válida
- `415` - Tipo de arquivo não suportado
- `422` - Documento inválido

**Extensões Válidas:**
- Arquivo assinado: `.pdf`, `.xml`, `.p7s`, `.json`
- Arquivo destacado: `.p7s`, `.jws`, `.xml`, `.p7m`, `.json`

---

### 2. POST /url

**Descrição:** Valida arquivo a partir de uma URL.

**Headers:**
```http
Content-Type: application/json
Referer: https://validar.iti.gov.br/
Accept: */*
Origin: https://validar.iti.gov.br
```

**Body (JSON):**
```json
{
  "url": "https://exemplo.com/arquivo.pdf"
}
```

**Resposta:** Similar ao `/arquivo`

---

### 3. POST /simples

**Descrição:** Processa os dados brutos e retorna relatório simplificado em JSON.

**Headers:**
```http
Content-Type: application/json
Accept: application/json
Referer: https://validar.iti.gov.br/
Origin: https://validar.iti.gov.br
Sec-Fetch-Site: same-origin
Sec-Fetch-Mode: cors
Sec-Fetch-Dest: empty
```

**Body (JSON):**
```json
{
  // Todo o JSON retornado pelo /arquivo
}
```

**Resposta (200 OK):**
```json
{
  "assinaturas": [
    {
      "nome": "João da Silva",
      "cpf": "12345678901",
      "certificadora": "AC XYZ",
      "numSerial": "ABC123",
      "data": "2025-11-22T10:30:00",
      "status": "VÁLIDO",
      "possuiCarimboTempo": true
    }
  ],
  "nomeArquivo": "documento.pdf",
  "hash": "a1b2c3d4...",
  "dataValidacao": "2025-11-22T15:45:00"
}
```

---

### 4. POST /conformidade

**Descrição:** Processa os dados brutos e retorna relatório de conformidade completo em JSON.

**Headers:**
```http
Content-Type: application/json
Accept: application/json
Referer: https://validar.iti.gov.br/
Origin: https://validar.iti.gov.br
Sec-Fetch-Site: same-origin
Sec-Fetch-Mode: cors
Sec-Fetch-Dest: empty
```

**Body (JSON):**
```json
{
  // Todo o JSON retornado pelo /arquivo
}
```

**Resposta (200 OK):**
```json
{
  "nomeArquivo": "documento.pdf",
  "assinaturas": [ ... ],
  // Dados mais completos de conformidade
  // Inclui validações da ICP-Brasil
  // Verificações de cadeia de certificação
}
```

**Diferença entre /simples e /conformidade:**
- `/simples`: Relatório resumido para uso programático
- `/conformidade`: Relatório detalhado usado para gerar o PDF

---

### 5. POST /downloadPdf ⭐

**Descrição:** Gera e retorna o PDF do relatório de conformidade.

**Headers:**
```http
Content-Type: application/json
Accept: application/json
Referer: https://validar.iti.gov.br/
Origin: https://validar.iti.gov.br
Sec-Fetch-Site: same-origin
Sec-Fetch-Mode: cors
Sec-Fetch-Dest: empty
```

**Body (JSON):**
```json
{
  "data": "{\"nomeArquivo\":\"doc.pdf\",\"assinaturas\":[...]}",
  "language": "pt-br"
}
```

**Estrutura do campo `data`:**
- É uma **string JSON** (JSON dentro de JSON)
- Contém o relatório de conformidade retornado por `/conformidade`
- Deve ser stringificado com `JSON.stringify()`

**Idiomas disponíveis:**
- `pt-br` - Português (Brasil)
- `en` - English
- `es` - Español

**Resposta (200 OK):**
```
Content-Type: application/pdf
[Binary PDF data]
```

**Nome padrão do arquivo:**
```
Relatorio - [nome_do_arquivo_original].pdf
```

---

### 6. POST /upload

**Descrição:** Envia documento para análise técnica do ITI.

**Headers:**
```http
Content-Type: multipart/form-data
```

**Body (multipart/form-data):**
```
name: Nome do usuário
email: email@exemplo.com
document: <arquivo>
```

---

## 🚦 Códigos de Status HTTP

| Código | Significado | Ação |
|--------|-------------|------|
| 200 | Sucesso | Documento validado com sucesso |
| 206 | Processamento Parcial | Algumas assinaturas válidas, outras não |
| 400 | Bad Request | Documento sem assinatura válida |
| 403 | Forbidden | Não autorizado |
| 404 | Not Found | Recurso não encontrado |
| 406 | Not Acceptable | Formato não aceito |
| 408 | Request Timeout | Tempo de processamento excedido |
| 415 | Unsupported Media Type | Tipo de arquivo não suportado |
| 422 | Unprocessable Entity | Documento inválido ou corrompido |
| 500 | Internal Server Error | Erro no servidor |
| 502 | Bad Gateway | Gateway com erro |
| 503 | Service Unavailable | Serviço temporariamente indisponível |

---

## 💻 Exemplos de Uso

### Exemplo 1: Validação Simples (Python)

```python
import requests
import json

# Passo 1: Enviar arquivo
url_arquivo = "https://validar.iti.gov.br/arquivo"
files = {'signature_files[]': ('documento.pdf', open('documento.pdf', 'rb'), 'application/pdf')}
headers = {'Referer': 'https://validar.iti.gov.br/'}

response1 = requests.post(url_arquivo, files=files, headers=headers)
json_bruto = response1.json()

# Passo 2: Obter relatório
url_simples = "https://validar.iti.gov.br/simples"
headers2 = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    'Referer': 'https://validar.iti.gov.br/'
}

response2 = requests.post(url_simples, json=json_bruto, headers=headers2)
relatorio = response2.json()

print(json.dumps(relatorio, indent=2, ensure_ascii=False))
```

---

### Exemplo 2: Download do PDF do Relatório (Python)

```python
import requests
import json

# Passo 1: Enviar arquivo
url_arquivo = "https://validar.iti.gov.br/arquivo"
files = {'signature_files[]': ('documento.pdf', open('documento.pdf', 'rb'), 'application/pdf')}
headers = {'Referer': 'https://validar.iti.gov.br/'}

response1 = requests.post(url_arquivo, files=files, headers=headers)
json_bruto = response1.json()

# Passo 2: Obter relatório de conformidade
url_conformidade = "https://validar.iti.gov.br/conformidade"
headers2 = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    'Referer': 'https://validar.iti.gov.br/'
}

response2 = requests.post(url_conformidade, json=json_bruto, headers=headers2)
relatorio_conformidade = response2.json()

# Passo 3: Baixar PDF
url_download = "https://validar.iti.gov.br/downloadPdf"
body = {
    "data": json.dumps(relatorio_conformidade),
    "language": "pt-br"
}

response3 = requests.post(url_download, json=body, headers=headers2)

# Salvar PDF
with open('Relatorio - documento.pdf', 'wb') as f:
    f.write(response3.content)

print("✓ PDF baixado com sucesso!")
```

---

### Exemplo 3: Validação por URL

```python
import requests
import json

# Passo 1: Validar por URL
url_validar = "https://validar.iti.gov.br/url"
body = {"url": "https://exemplo.com/documento.pdf"}
headers = {
    'Content-Type': 'application/json',
    'Referer': 'https://validar.iti.gov.br/'
}

response1 = requests.post(url_validar, json=body, headers=headers)
json_bruto = response1.json()

# Passo 2: Processar com /simples
url_simples = "https://validar.iti.gov.br/simples"
response2 = requests.post(url_simples, json=json_bruto, headers=headers)
relatorio = response2.json()

print(json.dumps(relatorio, indent=2, ensure_ascii=False))
```

---

### Exemplo 4: cURL

```bash
# Passo 1: Enviar arquivo
curl -X POST https://validar.iti.gov.br/arquivo \
  -H "Referer: https://validar.iti.gov.br/" \
  -F "signature_files[]=@documento.pdf" \
  -o resposta1.json

# Passo 2: Processar
curl -X POST https://validar.iti.gov.br/simples \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -H "Referer: https://validar.iti.gov.br/" \
  -d @resposta1.json \
  -o relatorio.json

# Passo 3: Download PDF (requer processamento do JSON)
# Primeiro obter relatório de conformidade
curl -X POST https://validar.iti.gov.br/conformidade \
  -H "Content-Type: application/json" \
  -H "Referer: https://validar.iti.gov.br/" \
  -d @resposta1.json \
  -o conformidade.json

# Depois fazer download (requer construir o body com jq)
jq -c '{data: (. | tostring), language: "pt-br"}' conformidade.json | \
curl -X POST https://validar.iti.gov.br/downloadPdf \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -H "Referer: https://validar.iti.gov.br/" \
  -d @- \
  -o relatorio.pdf
```

---

## 📝 Notas Importantes

1. **CORS:** Todos os endpoints exigem os headers corretos de CORS
2. **Referer:** O header `Referer` é obrigatório em todas as requisições
3. **Timeout:** Recomenda-se usar timeout de 60 segundos
4. **Armazenamento:** O ITI não armazena documentos enviados
5. **Rate Limiting:** Não há documentação oficial sobre limites de requisições
6. **Autenticação:** Não é necessária autenticação para uso público
7. **HTTPS:** Sempre use HTTPS para garantir segurança

---

## 🔍 Diferenças Entre Endpoints de Relatório

### /simples vs /conformidade

| Característica | /simples | /conformidade |
|----------------|----------|---------------|
| **Uso** | Relatório para processamento | Relatório para apresentação |
| **Detalhe** | Resumido | Completo |
| **Tamanho** | Menor | Maior |
| **PDF** | ❌ Não suporta | ✅ Usado para gerar PDF |
| **API** | ✅ Recomendado | Para download PDF |

---

## 🎯 Casos de Uso

### Quando usar /simples:
- Integração com sistemas
- Validação automática
- Processamento de lote
- Quando não precisa do PDF

### Quando usar /conformidade + /downloadPdf:
- Gerar comprovante visual
- Arquivamento
- Apresentação a terceiros
- Auditoria

---

## 🔗 Recursos Adicionais

- **Site oficial:** https://validar.iti.gov.br
- **Homologação:** https://h-validar.iti.gov.br
- **Glossário:** https://validar.iti.gov.br/glossarioRelatorioDeConformidade.html
- **Guia do Desenvolvedor:** https://validar.iti.gov.br/guia-desenvolvedor.html
- **Dúvidas:** https://validar.iti.gov.br/duvidas.html

---

## 📜 Licença e Avisos Legais

Esta documentação foi criada através de engenharia reversa do código JavaScript público do site validar.iti.gov.br.

**Aviso:** Esta é uma documentação não oficial. Para uso em produção, consulte a documentação oficial do ITI ou entre em contato com o suporte técnico.

**Data da análise:** 2025-11-22
**Versão do site analisada:** https://validar.iti.gov.br (versão de novembro/2025)

---

## 🐛 Troubleshooting

### Erro 400 - Documento sem assinatura
```
Causa: PDF não possui assinatura digital válida
Solução: Verifique se o documento está assinado digitalmente
```

### Erro 415 - Tipo não suportado
```
Causa: Extensão de arquivo inválida
Solução: Use apenas .pdf, .xml, .p7s ou .json
```

### Erro 422 - Documento inválido
```
Causa: Arquivo corrompido ou malformado
Solução: Verifique a integridade do arquivo
```

### PDF vazio ou corrompido no download
```
Causa: JSON do relatório de conformidade não foi stringificado corretamente
Solução: Use JSON.dumps() ou json.stringify() no campo "data"
```

### Timeout 408
```
Causa: Documento muito grande ou servidor sobrecarregado
Solução: Aumente o timeout ou tente novamente mais tarde
```

---

## 📊 Estrutura de Dados de Resposta

### Estrutura de Assinatura
```json
{
  "nome": "string",
  "cpf": "string",
  "certificadora": "string",
  "numSerial": "string",
  "data": "ISO 8601 datetime",
  "status": "VÁLIDO | INVÁLIDO | EXPIRADO",
  "possuiCarimboTempo": boolean
}
```

### Estrutura de Relatório Simples
```json
{
  "nomeArquivo": "string",
  "hash": "string",
  "dataValidacao": "ISO 8601 datetime",
  "statusDocumento": "string",
  "assinaturas": [
    { /* estrutura de assinatura */ }
  ]
}
```

---

**Última atualização:** 2025-11-22
**Contribuições:** Este documento pode ser atualizado conforme mudanças na API
