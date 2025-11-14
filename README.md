# Validador de Assinaturas PDF - ITI

Módulo Python para validação de assinaturas digitais em documentos PDF usando a API direta do Instituto Nacional de Tecnologia da Informação (ITI), sem usar Selenium.

## 🚀 Funcionalidades

- ✅ **Validação direta via API**: Comunicação direta com a API do ITI, sem necessidade de navegador
- ✅ **Modo silencioso e verboso**: Controle sobre a verbosidade da saída
- ✅ **Extração completa de dados**: Informações detalhadas sobre assinaturas, certificados e validade
- ✅ **Tratamento robusto de erros**: Detecção de documentos sem assinatura, erros de rede, etc.
- ✅ **Interface simples**: Uma única função `validate_pdf()` que retorna dados estruturados
- ✅ **Interface Gráfica Tkinter**: Interface visual amigável para uso desktop
- ✅ **Flexibilidade**: Use como API em seus projetos ou execute a interface gráfica

## 📦 Instalação

### Dependências

```bash
pip install -r requirements.txt
```

Ou instalar manualmente:

```bash
pip install requests
```

Para usar a interface gráfica, você também precisa do Tkinter (geralmente já incluído no Python):

```bash
# Ubuntu/Debian
sudo apt-get install python3-tk

# Fedora
sudo dnf install python3-tkinter

# macOS e Windows geralmente já vêm com Tkinter
```

## 💡 Uso Básico

### Opção 1: Interface Gráfica (GUI)

Execute a interface gráfica para validar PDFs de forma visual e intuitiva:

```bash
python3 tkinter_gui.py
```

![Interface Gráfica](https://github.com/user-attachments/assets/6803d849-0ba8-4ae2-9921-51480e8385d6)

**Como usar a GUI:**
1. Clique em "Selecionar PDF" para escolher um arquivo
2. Clique em "Validar Assinatura" para iniciar a validação
3. Veja os resultados detalhados na área de resultados
4. Use "Limpar" para começar uma nova validação

### Opção 2: API Python (Programático)

### Opção 2: API Python (Programático)

#### Importação

```python
from validator_api import validate_pdf
```

#### Validação Simples

```python
# Modo silencioso (padrão)
resultado = validate_pdf("meu_documento.pdf")
print(f"Status: {resultado['status']}")
```

#### Validação Detalhada

```python
# Modo verboso - mostra progresso
resultado = validate_pdf("meu_documento.pdf", verbose=True)
```

## 📋 Exemplos Completos

### Exemplo Básico

```python
from validator_api import validate_pdf

# Validar um PDF
resultado = validate_pdf("documento_assinado.pdf", verbose=True)

# Verificar resultado
if resultado['status'] == 'valid':
    print(f"✅ Válido! {resultado['total_assinaturas']} assinatura(s) encontrada(s)")
elif resultado['status'] == 'invalid':
    print("❌ Documento sem assinatura ou inválido")
else:
    print(f"⚠️ Erro: {resultado['error']}")
```

### Processar Assinaturas

```python
resultado = validate_pdf("documento.pdf")

if resultado['status'] == 'valid':
    print(f"Documento: {resultado['documento']['nome_arquivo']}")
    print(f"Hash: {resultado['documento']['hash']}")
    
    for i, assinatura in enumerate(resultado['assinaturas'], 1):
        print(f"\nAssinatura {i}:")
        print(f"  Assinado por: {assinatura['assinado_por']}")
        print(f"  CPF: {assinatura['cpf']}")
        print(f"  Certificadora: {assinatura['certificadora']}")
        print(f"  Status: {assinatura['status']}")
        print(f"  Carimbo do tempo: {'Sim' if assinatura['possui_carimbo_tempo'] else 'Não'}")
```

## 📊 Estrutura dos Dados

### Resultado de Documento Válido

```python
{
    "status": "valid",
    "documento": {
        "nome_arquivo": "documento.pdf",
        "hash": "abc123def456...",
        "data_validacao": "2025-11-06T10:30:00Z",
        "status_documento": "válido"
    },
    "assinaturas": [
        {
            "assinado_por": "João Silva Santos",
            "cpf": "123.456.789-00",
            "certificadora": "Autoridade Certificadora Raiz Brasileira v5",
            "numero_serie_certificado": "123456789ABCDEF",
            "data_assinatura": "2025-11-05T14:20:00Z",
            "status": "válida",
            "possui_carimbo_tempo": true
        }
    ],
    "total_assinaturas": 1,
    "relatorio_completo": {...}  # JSON bruto da API
}
```

### Resultado de Documento Inválido

```python
{
    "status": "invalid",
    "error": "Documento sem assinatura ou inválido",
    "details": {...}
}
```

### Resultado de Erro

```python
{
    "status": "error",
    "error": "Arquivo não encontrado: documento.pdf"
}
```

## 🎯 Status Possíveis

- `"valid"`: Documento possui uma ou mais assinaturas válidas
- `"invalid"`: Documento não possui assinatura ou assinatura inválida  
- `"error"`: Erro durante processamento (arquivo não encontrado, erro de rede, etc.)


## 🔧 Como Funciona

O módulo executa duas chamadas HTTP para a API do ITI:

1. **POST /arquivo**: Envia o PDF multipart/form-data e recebe um identificador
2. **POST /simples**: Envia o identificador JSON e recebe o relatório detalhado

### Headers Replicados

O módulo replica exatamente os headers do Chrome para compatibilidade com a API.

## ⚠️ Limitações

- Requer conexão com internet
- PDFs muito grandes podem causar timeout (60s)
- Depende da disponibilidade da API do ITI
- Não é uma API oficial (engenharia reversa)

## ��️ Desenvolvimento

### Estrutura do Projeto

```
validador_assinatura_iti/
├── validator_api.py          # Módulo principal (API)
├── tkinter_gui.py            # Interface gráfica Tkinter
├── requirements.txt          # Dependências
├── README.md                 # Este arquivo
└── .gitignore                # Arquivos ignorados
```

### Teste Rápido

**API Python:**
```bash
python3 -c "from validator_api import validate_pdf; print(validate_pdf('documento_assinado.pdf'))"
```

**Interface Gráfica:**
```bash
python3 tkinter_gui.py
```

## 📝 Licença

MIT License - use por sua conta e risco.

---

**Nota**: Este não é um projeto oficial do ITI. Desenvolvido através de engenharia reversa da interface web.
