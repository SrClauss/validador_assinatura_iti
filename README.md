# Validador de Assinaturas PDF - ITI# Validador de Assinaturas PDF - ITI# scrapper_iti



Este módulo permite validar assinaturas digitais de documentos PDF usando a API oficial do Instituto Nacional de Tecnologia da Informação (ITI).



## FuncionalidadesEste módulo permite validar assinaturas digitais de documentos PDF usando a API oficial do Instituto Nacional de Tecnologia da Informação (ITI).Script para validar assinaturas de PDFs no site do ITI usando Selenium (headless).



- ✅ Validação de assinaturas digitais em PDFs

- ✅ Extração de informações detalhadas das assinaturas

- ✅ Suporte a modo silencioso e verboso## Funcionalidades## Estrutura

- ✅ Detecção de documentos sem assinatura

- ✅ Tratamento robusto de erros- `src/iti_utils/validator.py`: funções reutilizáveis (`validate_signature`, `create_headless_chrome`).



## Instalação- ✅ Validação de assinaturas digitais em PDFs- `main.py`: executável que abre o site, rejeita cookies e valida arquivos informados.



### Dependências- ✅ Extração de informações detalhadas das assinaturas



```bash- ✅ Suporte a modo silencioso e verboso## Pré-requisitos

pip install requests

```- ✅ Detecção de documentos sem assinatura- Python 3.8+



O módulo usa apenas a biblioteca `requests` para fazer chamadas HTTP.- ✅ Tratamento robusto de erros- Google Chrome e ChromeDriver compatíveis no PATH (ou webdriver gerenciado no ambiente)



## Uso Básico- Selenium 4+



### Importação## Instalação



```python## Como executar

from validator_api import validate_pdf

```### DependênciasSem instalar o pacote (o `main.py` ajusta `sys.path` automaticamente):



### Modo Silencioso (Padrão)



```python```bash```bash

# Validação silenciosa - ideal para uso programático

resultado = validate_pdf("documento.pdf")pip install requestspython main.py "C00-Last Question.pdf" "FICHA_CNES_-_CHAIENY_assinado.pdf"

print(f"Status: {resultado['status']}")

`````````



### Modo Verboso



```pythonO módulo usa apenas a biblioteca `requests` para fazer chamadas HTTP.Se não passar argumentos, o script tentará validar exatamente os dois arquivos acima no diretório do repositório.

# Validação verbosa - mostra progresso detalhado

resultado = validate_pdf("documento.pdf", verbose=True)

```

## Uso BásicoSaída: imprime um JSON por arquivo, com `status: ok` e dados ou `status: error` e mensagem.

## Exemplos de Uso



### Exemplo Completo

### Importação## Uso como módulo

```python

from validator_api import validate_pdfOpcionalmente, instale em modo editável para importar `iti_utils` em outros projetos:



# Validar um PDF```python

resultado = validate_pdf("meu_documento.pdf", verbose=True)

from validator_api import validate_pdf```bash

# Verificar o status

if resultado['status'] == 'valid':```pip install -e ./src

    print(f"✅ Documento válido com {resultado['total_assinaturas']} assinatura(s)")

```

    # Listar informações das assinaturas

    for i, assinatura in enumerate(resultado['assinaturas'], 1):### Modo Silencioso (Padrão)

        print(f"{i}. {assinatura['assinado_por']} - {assinatura['status']}")

Então, em Python:

elif resultado['status'] == 'invalid':

    print("❌ Documento inválido ou sem assinatura")```python



else:# Validação silenciosa - ideal para uso programático```python

    print(f"⚠️ Erro: {resultado.get('error', 'Erro desconhecido')}")

```resultado = validate_pdf("documento.pdf")from iti_utils import create_headless_chrome, validate_signature



### Tratamento de Errosprint(f"Status: {resultado['status']}")



```python```driver = create_headless_chrome()

resultado = validate_pdf("arquivo_inexistente.pdf")

try:

if resultado['status'] == 'error':

    print(f"Erro: {resultado['error']}")### Modo Verboso    driver.get("https://validar.iti.gov.br/")

```

    result = validate_signature(driver, "/caminho/arquivo.pdf", timeout=25)

## Estrutura do Resultado

```python    print(result)

O módulo retorna um dicionário com a seguinte estrutura:

# Validação verbosa - mostra progresso detalhadofinally:

### Documento Válido

```pythonresultado = validate_pdf("documento.pdf", verbose=True)    driver.quit()

{

    "status": "valid",``````

    "documento": {

        "nome_arquivo": "documento.pdf",

        "hash": "abc123...",

        "data_validacao": "2025-11-06T...",## Exemplos de Uso## Dicas

        "status_documento": "válido"

    },- Em containers/CI, use `--no-sandbox` e `--disable-dev-shm-usage` (já habilitados em `create_headless_chrome`).

    "assinaturas": [

        {### Exemplo Completo- Se o site mudar seletores/fluxo, atualize `validator.py` conforme necessário.

            "assinado_por": "João Silva",

            "cpf": "123.456.789-00",

            "certificadora": "ICP-Brasil",```python

            "numero_serie_certificado": "ABC123...",from validator_api import validate_pdf

            "data_assinatura": "2025-11-05T...",

            "status": "válida",# Validar um PDF

            "possui_carimbo_tempo": trueresultado = validate_pdf("meu_documento.pdf", verbose=True)

        }

    ],# Verificar o status

    "total_assinaturas": 1,if resultado['status'] == 'valid':

    "relatorio_completo": {...}  # JSON bruto da API    print(f"✅ Documento válido com {resultado['total_assinaturas']} assinatura(s)")

}

```    # Listar informações das assinaturas

    for i, assinatura in enumerate(resultado['assinaturas'], 1):

### Documento Inválido/Sem Assinatura        print(f"{i}. {assinatura['assinado_por']} - {assinatura['status']}")

```python

{elif resultado['status'] == 'invalid':

    "status": "invalid",    print("❌ Documento inválido ou sem assinatura")

    "error": "Documento sem assinatura ou inválido",

    "details": {...}  # Detalhes do erro da APIelse:

}    print(f"⚠️ Erro: {resultado.get('error', 'Erro desconhecido')}")

``````



### Erro de Processamento### Tratamento de Erros

```python

{```python

    "status": "error",resultado = validate_pdf("arquivo_inexistente.pdf")

    "error": "Descrição do erro",

    "details": "Informações adicionais"if resultado['status'] == 'error':

}    print(f"Erro: {resultado['error']}")

``````



## Status Possíveis## Estrutura do Resultado



- `"valid"`: Documento válido com uma ou mais assinaturasO módulo retorna um dicionário com a seguinte estrutura:

- `"invalid"`: Documento sem assinatura ou com assinatura inválida

- `"error"`: Erro durante o processamento (rede, arquivo não encontrado, etc.)### Documento Válido

```python

## Arquivos de Teste{

    "status": "valid",

O repositório inclui alguns arquivos PDF de exemplo:    "documento": {

        "nome_arquivo": "documento.pdf",

- `FICHA_CNES_-_CHAIENY_assinado.pdf`: PDF com 1 assinatura válida        "hash": "abc123...",

- `FICHA_CNES_-_CHAIENY_assinado_assinado.pdf`: PDF com 2 assinaturas válidas        "data_validacao": "2025-11-06T...",

- `C00-Last Question.pdf`: PDF sem assinatura (para teste de documento inválido)        "status_documento": "válido"

    },

## Como Funciona    "assinaturas": [

        {

O módulo faz duas chamadas HTTP para a API do ITI:            "assinado_por": "João Silva",

            "cpf": "123.456.789-00",

1. **POST /arquivo**: Envia o PDF e recebe um identificador            "certificadora": "ICP-Brasil",

2. **POST /simples**: Processa o identificador e retorna o relatório de validação            "numero_serie_certificado": "ABC123...",

            "data_assinatura": "2025-11-05T...",

### Headers e Autenticação            "status": "válida",

            "possui_carimbo_tempo": true

O módulo replica exatamente os headers enviados pelo navegador Chrome para garantir compatibilidade com a API.        }

    ],

## Limitações    "total_assinaturas": 1,

    "relatorio_completo": {...}  # JSON bruto da API

- Requer conexão com internet para acessar a API do ITI}

- PDFs muito grandes podem causar timeouts (limite atual: 60 segundos)```

- Dependente da disponibilidade da API do ITI

### Documento Inválido/Sem Assinatura

## Desenvolvimento```python

{

### Estrutura do Projeto    "status": "invalid",

    "error": "Documento sem assinatura ou inválido",

```    "details": {...}  # Detalhes do erro da API

scrapper_iti/}

├── validator_api.py      # Módulo principal```

├── METODO_CAPTURA_REQUISICOES.md  # Documentação da engenharia reversa

└── README.md            # Este arquivo### Erro de Processamento

``````python

{

## Contribuição    "status": "error",

    "error": "Descrição do erro",

Para contribuir:    "details": "Informações adicionais"

}

1. Fork o projeto```

2. Crie uma branch para sua feature (`git checkout -b feature/nova-funcionalidade`)

3. Commit suas mudanças (`git commit -am 'Adiciona nova funcionalidade'`)## Status Possíveis

4. Push para a branch (`git push origin feature/nova-funcionalidade`)

5. Abra um Pull Request- `"valid"`: Documento válido com uma ou mais assinaturas

- `"invalid"`: Documento sem assinatura ou com assinatura inválida

## Licença- `"error"`: Erro durante o processamento (rede, arquivo não encontrado, etc.)



Este projeto é distribuído sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.## Arquivos de Teste



## SuporteO repositório inclui alguns arquivos PDF de exemplo:



Para reportar bugs ou solicitar features, abra uma issue no GitHub.- `FICHA_CNES_-_CHAIENY_assinado.pdf`: PDF com 1 assinatura válida

- `FICHA_CNES_-_CHAIENY_assinado_assinado.pdf`: PDF com 2 assinaturas válidas

---- `C00-Last Question.pdf`: PDF sem assinatura (para teste de documento inválido)



**Nota**: Este módulo não é oficial do ITI e foi desenvolvido através de engenharia reversa da interface web. Use por sua própria conta e risco.</content>## Desenvolvimento

<parameter name="filePath">/home/claus/src/scrapper_iti/README.md
### Estrutura do Projeto

```
scrapper_iti/
├── validator_api.py      # Módulo principal
├── main.py              # Script original com Selenium (backup)
├── METODO_CAPTURA_REQUISICOES.md  # Documentação da engenharia reversa
└── README.md            # Este arquivo
```

### Como Funciona

O módulo faz duas chamadas HTTP para a API do ITI:

1. **POST /arquivo**: Envia o PDF e recebe um identificador
2. **POST /simples**: Processa o identificador e retorna o relatório de validação

### Headers e Autenticação

O módulo replica exatamente os headers enviados pelo navegador Chrome para garantir compatibilidade com a API.

## Limitações

- Requer conexão com internet para acessar a API do ITI
- PDFs muito grandes podem causar timeouts (limite atual: 60 segundos)
- Dependente da disponibilidade da API do ITI

## Contribuição

Para contribuir:

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

## Licença

Este projeto é distribuído sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.

## Suporte

Para reportar bugs ou solicitar features, abra uma issue no GitHub.

---

**Nota**: Este módulo não é oficial do ITI e foi desenvolvido através de engenharia reversa da interface web. Use por sua própria conta e risco.</content>
<parameter name="filePath">/home/claus/src/scrapper_iti/README.md