# Integração com APIs REST

Guia completo para integrar as funções de validação e download de PDF em APIs REST.

## 📋 Sumário

- [Funções Disponíveis](#funções-disponíveis)
- [Flask](#integração-com-flask)
- [FastAPI](#integração-com-fastapi)
- [Exemplos de Uso](#exemplos-de-uso-da-api)
- [Tratamento de Erros](#tratamento-de-erros)
- [Boas Práticas](#boas-práticas)

---

## Funções Disponíveis

### 1. `validate_pdf(pdf_path, verbose=False)`

Valida assinaturas e retorna dados estruturados (relatório simplificado).

**Retorna:**
```python
{
    "status": "valid",  # ou "invalid", "error"
    "documento": {
        "nome_arquivo": "doc.pdf",
        "hash": "abc123...",
        "data_validacao": "2025-11-22T10:30:00Z",
        "status_documento": "válido"
    },
    "assinaturas": [
        {
            "assinado_por": "João Silva",
            "cpf": "123.456.789-00",
            "certificadora": "AC Raiz v5",
            "numero_serie_certificado": "ABC123",
            "data_assinatura": "2025-11-20T14:20:00Z",
            "status": "válida",
            "possui_carimbo_tempo": true
        }
    ],
    "total_assinaturas": 1
}
```

### 2. `get_conformidade_report(pdf_path, verbose=False)`

Obtém relatório de conformidade completo (necessário para gerar PDF).

**Retorna:**
```python
{
    "status": "success",  # ou "invalid", "error"
    "relatorio_conformidade": {...},  # JSON completo
    "json_bruto": {...}
}
```

### 3. `download_relatorio_pdf(relatorio_conformidade, language="pt-br", save_as=None, verbose=False)`

Baixa o PDF do relatório de validação.

**Parâmetros:**
- `relatorio_conformidade`: JSON do relatório (de `get_conformidade_report()`)
- `language`: `"pt-br"`, `"en"` ou `"es"`
- `save_as`: Caminho para salvar (ou `None` para retornar bytes)
- `verbose`: Exibir progresso

**Retorna:**
```python
{
    "status": "success",
    "pdf_bytes": b"...",
    "pdf_path": "/path/to/saved.pdf"  # se save_as foi fornecido
}
```

---

## Integração com Flask

### 1. Instalação

```bash
pip install flask
pip install requests
```

### 2. Estrutura do Projeto

```
projeto/
├── app.py                    # API Flask
├── validator_api.py          # Funções do validador
├── uploads/                  # PDFs temporários
└── relatorios/              # PDFs de relatório gerados
```

### 3. Código Completo (`app.py`)

```python
from flask import Flask, request, jsonify, send_file
from werkzeug.utils import secure_filename
import os
from pathlib import Path
from validator_api import validate_pdf, get_conformidade_report, download_relatorio_pdf

app = Flask(__name__)
app.config['UPLOAD_FOLDER'] = 'uploads'
app.config['RELATORIO_FOLDER'] = 'relatorios'
app.config['MAX_CONTENT_LENGTH'] = 50 * 1024 * 1024  # 50MB max

# Criar diretórios se não existirem
Path(app.config['UPLOAD_FOLDER']).mkdir(exist_ok=True)
Path(app.config['RELATORIO_FOLDER']).mkdir(exist_ok=True)

ALLOWED_EXTENSIONS = {'pdf'}


def allowed_file(filename):
    return '.' in filename and filename.rsplit('.', 1)[1].lower() in ALLOWED_EXTENSIONS


@app.route('/api/validar', methods=['POST'])
def validar_pdf():
    """
    Valida assinatura de PDF e retorna JSON.

    Body: multipart/form-data
    - file: arquivo PDF

    Returns: JSON com resultado da validação
    """
    if 'file' not in request.files:
        return jsonify({"error": "Nenhum arquivo enviado"}), 400

    file = request.files['file']

    if file.filename == '':
        return jsonify({"error": "Nome de arquivo vazio"}), 400

    if not allowed_file(file.filename):
        return jsonify({"error": "Apenas arquivos PDF são permitidos"}), 400

    try:
        # Salvar arquivo temporariamente
        filename = secure_filename(file.filename)
        filepath = os.path.join(app.config['UPLOAD_FOLDER'], filename)
        file.save(filepath)

        # Validar
        resultado = validate_pdf(filepath, verbose=False)

        # Limpar arquivo temporário
        os.remove(filepath)

        return jsonify(resultado), 200

    except Exception as e:
        return jsonify({"error": str(e)}), 500


@app.route('/api/validar-com-pdf', methods=['POST'])
def validar_e_gerar_pdf():
    """
    Valida assinatura e retorna JSON + gera PDF do relatório.

    Body: multipart/form-data
    - file: arquivo PDF
    - language: (opcional) "pt-br", "en" ou "es" (padrão: "pt-br")

    Returns: JSON com resultado + link para download do PDF
    """
    if 'file' not in request.files:
        return jsonify({"error": "Nenhum arquivo enviado"}), 400

    file = request.files['file']
    language = request.form.get('language', 'pt-br')

    if file.filename == '':
        return jsonify({"error": "Nome de arquivo vazio"}), 400

    if not allowed_file(file.filename):
        return jsonify({"error": "Apenas arquivos PDF são permitidos"}), 400

    if language not in ['pt-br', 'en', 'es']:
        return jsonify({"error": "Idioma inválido. Use pt-br, en ou es"}), 400

    try:
        # Salvar arquivo temporariamente
        filename = secure_filename(file.filename)
        filepath = os.path.join(app.config['UPLOAD_FOLDER'], filename)
        file.save(filepath)

        # 1. Validação simples
        validacao = validate_pdf(filepath, verbose=False)

        # 2. Obter relatório de conformidade
        relatorio = get_conformidade_report(filepath, verbose=False)

        if relatorio['status'] != 'success':
            os.remove(filepath)
            return jsonify({
                "validacao": validacao,
                "erro_relatorio": relatorio.get('error')
            }), 400

        # 3. Gerar PDF do relatório
        pdf_filename = f"Relatorio_{filename}"
        pdf_path = os.path.join(app.config['RELATORIO_FOLDER'], pdf_filename)

        pdf_result = download_relatorio_pdf(
            relatorio['relatorio_conformidade'],
            language=language,
            save_as=pdf_path,
            verbose=False
        )

        # Limpar arquivo temporário
        os.remove(filepath)

        if pdf_result['status'] != 'success':
            return jsonify({
                "validacao": validacao,
                "erro_pdf": pdf_result.get('error')
            }), 500

        return jsonify({
            "validacao": validacao,
            "relatorio_pdf": {
                "download_url": f"/api/download-relatorio/{pdf_filename}",
                "filename": pdf_filename
            }
        }), 200

    except Exception as e:
        if os.path.exists(filepath):
            os.remove(filepath)
        return jsonify({"error": str(e)}), 500


@app.route('/api/download-relatorio/<filename>', methods=['GET'])
def download_relatorio(filename):
    """
    Faz download do PDF do relatório gerado.

    Params:
    - filename: nome do arquivo

    Returns: PDF file
    """
    try:
        filename = secure_filename(filename)
        filepath = os.path.join(app.config['RELATORIO_FOLDER'], filename)

        if not os.path.exists(filepath):
            return jsonify({"error": "Arquivo não encontrado"}), 404

        return send_file(
            filepath,
            mimetype='application/pdf',
            as_attachment=True,
            download_name=filename
        )

    except Exception as e:
        return jsonify({"error": str(e)}), 500


@app.route('/api/relatorio-pdf-bytes', methods=['POST'])
def obter_pdf_bytes():
    """
    Valida e retorna PDF do relatório como bytes (base64).
    Útil para integrações que precisam do arquivo direto.

    Body: multipart/form-data
    - file: arquivo PDF
    - language: (opcional) "pt-br", "en" ou "es"

    Returns: JSON com PDF em base64
    """
    import base64

    if 'file' not in request.files:
        return jsonify({"error": "Nenhum arquivo enviado"}), 400

    file = request.files['file']
    language = request.form.get('language', 'pt-br')

    if file.filename == '':
        return jsonify({"error": "Nome de arquivo vazio"}), 400

    try:
        # Salvar temporariamente
        filename = secure_filename(file.filename)
        filepath = os.path.join(app.config['UPLOAD_FOLDER'], filename)
        file.save(filepath)

        # Obter relatório
        relatorio = get_conformidade_report(filepath, verbose=False)

        if relatorio['status'] != 'success':
            os.remove(filepath)
            return jsonify({"error": relatorio.get('error')}), 400

        # Baixar PDF (sem salvar)
        pdf_result = download_relatorio_pdf(
            relatorio['relatorio_conformidade'],
            language=language,
            save_as=None,  # Não salva
            verbose=False
        )

        os.remove(filepath)

        if pdf_result['status'] != 'success':
            return jsonify({"error": pdf_result.get('error')}), 500

        # Converter para base64
        pdf_base64 = base64.b64encode(pdf_result['pdf_bytes']).decode('utf-8')

        return jsonify({
            "pdf_base64": pdf_base64,
            "filename": f"Relatorio_{filename}"
        }), 200

    except Exception as e:
        if os.path.exists(filepath):
            os.remove(filepath)
        return jsonify({"error": str(e)}), 500


if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
```

### 4. Executar

```bash
python app.py
```

A API estará disponível em `http://localhost:5000`

---

## Integração com FastAPI

### 1. Instalação

```bash
pip install fastapi uvicorn python-multipart
pip install requests
```

### 2. Código Completo (`main.py`)

```python
from fastapi import FastAPI, File, UploadFile, Form, HTTPException
from fastapi.responses import FileResponse, JSONResponse
from pathlib import Path
import shutil
import base64
from typing import Optional
from validator_api import validate_pdf, get_conformidade_report, download_relatorio_pdf

app = FastAPI(
    title="API Validador ITI",
    description="API para validação de assinaturas digitais em PDFs",
    version="1.0.0"
)

UPLOAD_FOLDER = Path("uploads")
RELATORIO_FOLDER = Path("relatorios")

# Criar diretórios
UPLOAD_FOLDER.mkdir(exist_ok=True)
RELATORIO_FOLDER.mkdir(exist_ok=True)


@app.post("/api/validar", summary="Valida assinatura de PDF")
async def validar_pdf_endpoint(file: UploadFile = File(...)):
    """
    Valida assinatura digital de um PDF.

    - **file**: Arquivo PDF para validar

    Retorna JSON com informações da validação.
    """
    if not file.filename.endswith('.pdf'):
        raise HTTPException(status_code=400, detail="Apenas arquivos PDF são permitidos")

    filepath = UPLOAD_FOLDER / file.filename

    try:
        # Salvar arquivo temporário
        with filepath.open("wb") as buffer:
            shutil.copyfileobj(file.file, buffer)

        # Validar
        resultado = validate_pdf(str(filepath), verbose=False)

        return resultado

    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

    finally:
        # Limpar
        if filepath.exists():
            filepath.unlink()


@app.post("/api/validar-com-pdf", summary="Valida e gera PDF do relatório")
async def validar_e_gerar_pdf_endpoint(
    file: UploadFile = File(...),
    language: str = Form("pt-br")
):
    """
    Valida assinatura e gera PDF do relatório.

    - **file**: Arquivo PDF para validar
    - **language**: Idioma do relatório (pt-br, en, es)

    Retorna JSON com validação + link para download do relatório.
    """
    if not file.filename.endswith('.pdf'):
        raise HTTPException(status_code=400, detail="Apenas arquivos PDF são permitidos")

    if language not in ['pt-br', 'en', 'es']:
        raise HTTPException(status_code=400, detail="Idioma inválido")

    filepath = UPLOAD_FOLDER / file.filename

    try:
        # Salvar temporário
        with filepath.open("wb") as buffer:
            shutil.copyfileobj(file.file, buffer)

        # 1. Validação
        validacao = validate_pdf(str(filepath), verbose=False)

        # 2. Relatório de conformidade
        relatorio = get_conformidade_report(str(filepath), verbose=False)

        if relatorio['status'] != 'success':
            raise HTTPException(
                status_code=400,
                detail=f"Erro ao obter relatório: {relatorio.get('error')}"
            )

        # 3. Gerar PDF
        pdf_filename = f"Relatorio_{file.filename}"
        pdf_path = RELATORIO_FOLDER / pdf_filename

        pdf_result = download_relatorio_pdf(
            relatorio['relatorio_conformidade'],
            language=language,
            save_as=str(pdf_path),
            verbose=False
        )

        if pdf_result['status'] != 'success':
            raise HTTPException(
                status_code=500,
                detail=f"Erro ao gerar PDF: {pdf_result.get('error')}"
            )

        return {
            "validacao": validacao,
            "relatorio_pdf": {
                "download_url": f"/api/download-relatorio/{pdf_filename}",
                "filename": pdf_filename
            }
        }

    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

    finally:
        if filepath.exists():
            filepath.unlink()


@app.get("/api/download-relatorio/{filename}", summary="Download do PDF do relatório")
async def download_relatorio_endpoint(filename: str):
    """
    Faz download do PDF do relatório gerado.

    - **filename**: Nome do arquivo
    """
    filepath = RELATORIO_FOLDER / filename

    if not filepath.exists():
        raise HTTPException(status_code=404, detail="Arquivo não encontrado")

    return FileResponse(
        path=filepath,
        media_type='application/pdf',
        filename=filename
    )


@app.post("/api/relatorio-pdf-bytes", summary="Retorna PDF como base64")
async def obter_pdf_bytes_endpoint(
    file: UploadFile = File(...),
    language: str = Form("pt-br")
):
    """
    Valida e retorna PDF do relatório como base64.

    - **file**: Arquivo PDF
    - **language**: Idioma (pt-br, en, es)
    """
    if not file.filename.endswith('.pdf'):
        raise HTTPException(status_code=400, detail="Apenas PDFs são permitidos")

    filepath = UPLOAD_FOLDER / file.filename

    try:
        # Salvar temporário
        with filepath.open("wb") as buffer:
            shutil.copyfileobj(file.file, buffer)

        # Obter relatório
        relatorio = get_conformidade_report(str(filepath), verbose=False)

        if relatorio['status'] != 'success':
            raise HTTPException(status_code=400, detail=relatorio.get('error'))

        # Baixar PDF
        pdf_result = download_relatorio_pdf(
            relatorio['relatorio_conformidade'],
            language=language,
            save_as=None,
            verbose=False
        )

        if pdf_result['status'] != 'success':
            raise HTTPException(status_code=500, detail=pdf_result.get('error'))

        # Base64
        pdf_base64 = base64.b64encode(pdf_result['pdf_bytes']).decode('utf-8')

        return {
            "pdf_base64": pdf_base64,
            "filename": f"Relatorio_{file.filename}"
        }

    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

    finally:
        if filepath.exists():
            filepath.unlink()


@app.get("/", summary="Health check")
async def root():
    return {"status": "ok", "message": "API Validador ITI"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

### 3. Executar

```bash
uvicorn main:app --reload
```

Acesse a documentação interativa em: `http://localhost:8000/docs`

---

## Exemplos de Uso da API

### 1. Validar PDF apenas (JSON)

**cURL:**
```bash
curl -X POST http://localhost:5000/api/validar \
  -F "file=@documento.pdf"
```

**Python (requests):**
```python
import requests

with open('documento.pdf', 'rb') as f:
    response = requests.post(
        'http://localhost:5000/api/validar',
        files={'file': f}
    )

print(response.json())
```

**JavaScript (fetch):**
```javascript
const formData = new FormData();
formData.append('file', fileInput.files[0]);

const response = await fetch('http://localhost:5000/api/validar', {
    method: 'POST',
    body: formData
});

const data = await response.json();
console.log(data);
```

### 2. Validar + Gerar PDF do Relatório

**cURL:**
```bash
curl -X POST http://localhost:5000/api/validar-com-pdf \
  -F "file=@documento.pdf" \
  -F "language=pt-br"
```

**Python:**
```python
import requests

with open('documento.pdf', 'rb') as f:
    response = requests.post(
        'http://localhost:5000/api/validar-com-pdf',
        files={'file': f},
        data={'language': 'pt-br'}
    )

result = response.json()
download_url = result['relatorio_pdf']['download_url']
print(f"PDF disponível em: {download_url}")
```

### 3. Download do Relatório

**cURL:**
```bash
curl -O http://localhost:5000/api/download-relatorio/Relatorio_documento.pdf
```

**Python:**
```python
import requests

response = requests.get(
    'http://localhost:5000/api/download-relatorio/Relatorio_documento.pdf'
)

with open('relatorio_baixado.pdf', 'wb') as f:
    f.write(response.content)
```

### 4. Obter PDF como Base64

**Python:**
```python
import requests
import base64

with open('documento.pdf', 'rb') as f:
    response = requests.post(
        'http://localhost:5000/api/relatorio-pdf-bytes',
        files={'file': f},
        data={'language': 'en'}
    )

result = response.json()
pdf_bytes = base64.b64decode(result['pdf_base64'])

with open('relatorio.pdf', 'wb') as f:
    f.write(pdf_bytes)
```

---

## Tratamento de Erros

### Códigos de Status HTTP

| Código | Significado | Quando Ocorre |
|--------|-------------|---------------|
| 200 | Sucesso | Validação concluída com sucesso |
| 400 | Bad Request | Arquivo inválido, parâmetros incorretos |
| 404 | Not Found | Arquivo de relatório não encontrado |
| 500 | Internal Server Error | Erro ao processar requisição |

### Exemplo de Resposta de Erro

```json
{
    "error": "Documento sem assinatura ou inválido"
}
```

### Tratamento em Python

```python
import requests

try:
    response = requests.post('http://localhost:5000/api/validar', files={'file': f})
    response.raise_for_status()  # Levanta exceção se status >= 400

    resultado = response.json()

    if resultado['status'] == 'valid':
        print(f"✅ Válido: {resultado['total_assinaturas']} assinaturas")
    elif resultado['status'] == 'invalid':
        print(f"❌ Inválido: {resultado.get('error')}")
    else:
        print(f"⚠️ Erro: {resultado.get('error')}")

except requests.exceptions.RequestException as e:
    print(f"Erro de rede: {e}")
```

---

## Boas Práticas

### 1. Segurança

```python
# ✅ BOM: Validar extensão do arquivo
if not filename.endswith('.pdf'):
    raise HTTPException(status_code=400, detail="Apenas PDFs")

# ✅ BOM: Usar secure_filename
from werkzeug.utils import secure_filename
filename = secure_filename(file.filename)

# ✅ BOM: Limitar tamanho do arquivo
app.config['MAX_CONTENT_LENGTH'] = 50 * 1024 * 1024  # 50MB
```

### 2. Limpeza de Arquivos Temporários

```python
# ✅ BOM: Sempre limpar no finally
try:
    # processar arquivo
    pass
finally:
    if os.path.exists(filepath):
        os.remove(filepath)
```

### 3. Processamento Assíncrono (para grandes volumes)

```python
# Para FastAPI com Celery
from celery import Celery

celery_app = Celery('tasks', broker='redis://localhost:6379')

@celery_app.task
def processar_pdf_async(filepath, language):
    relatorio = get_conformidade_report(filepath)
    return download_relatorio_pdf(
        relatorio['relatorio_conformidade'],
        language=language
    )

@app.post("/api/validar-async")
async def validar_async(file: UploadFile):
    # Salvar arquivo
    task = processar_pdf_async.delay(filepath, 'pt-br')
    return {"task_id": task.id}
```

### 4. Cache de Relatórios

```python
import hashlib
from functools import lru_cache

def get_file_hash(filepath):
    with open(filepath, 'rb') as f:
        return hashlib.sha256(f.read()).hexdigest()

# Cache baseado no hash do arquivo
@lru_cache(maxsize=100)
def get_cached_report(file_hash):
    # Buscar relatório em cache/banco
    pass
```

### 5. Logging

```python
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@app.post("/api/validar")
async def validar(file: UploadFile):
    logger.info(f"Validando arquivo: {file.filename}")
    try:
        resultado = validate_pdf(filepath)
        logger.info(f"Validação concluída: {resultado['status']}")
        return resultado
    except Exception as e:
        logger.error(f"Erro na validação: {e}", exc_info=True)
        raise
```

---

## Exemplo Completo: Cliente Python

```python
import requests
from pathlib import Path

class ValidadorITIClient:
    def __init__(self, base_url="http://localhost:5000"):
        self.base_url = base_url

    def validar(self, pdf_path):
        """Valida PDF e retorna JSON"""
        with open(pdf_path, 'rb') as f:
            response = requests.post(
                f"{self.base_url}/api/validar",
                files={'file': f}
            )
        response.raise_for_status()
        return response.json()

    def validar_e_baixar_relatorio(self, pdf_path, output_dir=".", language="pt-br"):
        """Valida e baixa PDF do relatório"""
        # 1. Validar e gerar
        with open(pdf_path, 'rb') as f:
            response = requests.post(
                f"{self.base_url}/api/validar-com-pdf",
                files={'file': f},
                data={'language': language}
            )
        response.raise_for_status()
        result = response.json()

        # 2. Baixar PDF
        download_url = result['relatorio_pdf']['download_url']
        pdf_response = requests.get(f"{self.base_url}{download_url}")

        # 3. Salvar
        output_path = Path(output_dir) / result['relatorio_pdf']['filename']
        output_path.write_bytes(pdf_response.content)

        return {
            'validacao': result['validacao'],
            'pdf_path': str(output_path)
        }

# Uso
client = ValidadorITIClient()

# Validação simples
resultado = client.validar("documento.pdf")
print(f"Status: {resultado['status']}")

# Com relatório PDF
resultado = client.validar_e_baixar_relatorio("documento.pdf", output_dir="relatorios")
print(f"PDF salvo em: {resultado['pdf_path']}")
```

---

## Notas Finais

1. **Timeouts**: As requisições ao ITI podem demorar. Configure timeouts adequados (60s+)
2. **Rate Limiting**: Considere implementar rate limiting para evitar sobrecarga
3. **Armazenamento**: PDFs de relatório podem acumular. Implemente rotina de limpeza
4. **Autenticação**: Para produção, adicione autenticação (JWT, API keys, etc.)
5. **CORS**: Configure CORS se a API for consumida por frontend web

```python
# Flask CORS
from flask_cors import CORS
CORS(app)

# FastAPI CORS
from fastapi.middleware.cors import CORSMiddleware
app.add_middleware(CORSMiddleware, allow_origins=["*"])
```
