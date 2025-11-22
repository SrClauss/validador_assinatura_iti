"""
Exemplo de como fazer download do PDF de relatório de validação do ITI.

Este script demonstra o fluxo completo:
1. POST /arquivo - envia PDF e recebe identificador
2. POST /conformidade - processa e retorna relatório de conformidade
3. POST /downloadPdf - gera e baixa o PDF do relatório

Baseado na análise do código JavaScript do site validar.iti.gov.br
"""

import json
import requests
from pathlib import Path


def validar_e_baixar_relatorio_pdf(pdf_path, output_path=None, language="pt-br", verbose=False):
    """
    Valida PDF e baixa o relatório de conformidade em PDF.

    Args:
        pdf_path: Caminho para o arquivo PDF a ser validado
        output_path: Caminho para salvar o relatório PDF (opcional)
        language: Idioma do relatório (pt-br, en, es)
        verbose: Se True, mostra mensagens de progresso

    Returns:
        dict com status e informações do download
    """
    pdf_path = Path(pdf_path)
    if not pdf_path.exists():
        raise FileNotFoundError(f"Arquivo não encontrado: {pdf_path}")

    # Define o caminho de saída padrão
    if output_path is None:
        output_path = pdf_path.parent / f"Relatorio - {pdf_path.stem}.pdf"
    else:
        output_path = Path(output_path)

    if verbose:
        print(f"\n{'='*60}")
        print(f"Validando: {pdf_path.name}")
        print(f"{'='*60}\n")

    # ========================================
    # PASSO 1: POST /arquivo
    # ========================================
    url_arquivo = "https://validar.iti.gov.br/arquivo"
    headers_arquivo = {
        'Referer': 'https://validar.iti.gov.br/',
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36',
        'sec-ch-ua': '"Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99"',
        'sec-ch-ua-mobile': '?0',
        'sec-ch-ua-platform': '"Linux"',
        'Accept': '*/*',
        'Origin': 'https://validar.iti.gov.br',
        'Sec-Fetch-Site': 'same-origin',
        'Sec-Fetch-Mode': 'cors',
        'Sec-Fetch-Dest': 'empty',
    }

    files = {
        'signature_files[]': (pdf_path.name, pdf_path.open('rb'), 'application/pdf')
    }

    if verbose:
        print("📤 [1/3] Enviando PDF para /arquivo...")

    try:
        response_arquivo = requests.post(url_arquivo, headers=headers_arquivo, files=files, timeout=60)

        if verbose:
            print(f"   Status: {response_arquivo.status_code}")

        if response_arquivo.status_code == 400:
            return {
                "status": "invalid",
                "error": "Documento sem assinatura ou inválido",
                "details": response_arquivo.json() if response_arquivo.content else None
            }

        if response_arquivo.status_code != 200:
            return {
                "status": "error",
                "error": f"Erro HTTP {response_arquivo.status_code} em /arquivo",
                "details": response_arquivo.text
            }

        json_bruto = response_arquivo.json()

        if verbose:
            print(f"   ✓ Resposta recebida ({len(json.dumps(json_bruto))} bytes)")

    except Exception as e:
        return {
            "status": "error",
            "error": f"Erro ao enviar arquivo: {str(e)}"
        }
    finally:
        files['signature_files[]'][1].close()

    # ========================================
    # PASSO 2: POST /conformidade
    # ========================================
    url_conformidade = "https://validar.iti.gov.br/conformidade"
    headers_conformidade = {
        'Accept': 'application/json',
        'Content-Type': 'application/json',
        'Referer': 'https://validar.iti.gov.br/',
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36',
        'sec-ch-ua': '"Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99"',
        'sec-ch-ua-mobile': '?0',
        'sec-ch-ua-platform': '"Linux"',
        'Origin': 'https://validar.iti.gov.br',
        'Sec-Fetch-Site': 'same-origin',
        'Sec-Fetch-Mode': 'cors',
        'Sec-Fetch-Dest': 'empty',
    }

    if verbose:
        print("📋 [2/3] Processando com /conformidade...")

    try:
        response_conformidade = requests.post(
            url_conformidade,
            headers=headers_conformidade,
            json=json_bruto,
            timeout=60
        )

        if verbose:
            print(f"   Status: {response_conformidade.status_code}")

        if response_conformidade.status_code != 200:
            return {
                "status": "error",
                "error": f"Erro HTTP {response_conformidade.status_code} em /conformidade",
                "json_bruto": json_bruto,
                "details": response_conformidade.text
            }

        relatorio_conformidade = response_conformidade.json()

        if verbose:
            print(f"   ✓ Relatório de conformidade recebido")

    except Exception as e:
        return {
            "status": "error",
            "error": f"Erro ao processar /conformidade: {str(e)}",
            "json_bruto": json_bruto
        }

    # ========================================
    # PASSO 3: POST /downloadPdf
    # ========================================
    url_download_pdf = "https://validar.iti.gov.br/downloadPdf"
    headers_download = {
        'Accept': 'application/json',
        'Content-Type': 'application/json',
        'Referer': 'https://validar.iti.gov.br/',
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36',
        'sec-ch-ua': '"Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99"',
        'sec-ch-ua-mobile': '?0',
        'sec-ch-ua-platform': '"Linux"',
        'Origin': 'https://validar.iti.gov.br',
        'Sec-Fetch-Site': 'same-origin',
        'Sec-Fetch-Mode': 'cors',
        'Sec-Fetch-Dest': 'empty',
    }

    # Corpo da requisição conforme documentado no código JavaScript
    request_body = {
        "data": json.dumps(relatorio_conformidade),
        "language": language
    }

    if verbose:
        print("📥 [3/3] Baixando PDF do relatório...")

    try:
        response_pdf = requests.post(
            url_download_pdf,
            headers=headers_download,
            json=request_body,
            timeout=60
        )

        if verbose:
            print(f"   Status: {response_pdf.status_code}")

        if response_pdf.status_code != 200:
            return {
                "status": "error",
                "error": f"Erro HTTP {response_pdf.status_code} em /downloadPdf",
                "details": response_pdf.text
            }

        # Salva o PDF
        with open(output_path, 'wb') as f:
            f.write(response_pdf.content)

        file_size = len(response_pdf.content)

        if verbose:
            print(f"   ✓ PDF salvo: {output_path}")
            print(f"   Tamanho: {file_size:,} bytes ({file_size/1024:.2f} KB)")
            print(f"\n{'='*60}")
            print(f"✓ Relatório PDF gerado com sucesso!")
            print(f"{'='*60}\n")

        return {
            "status": "success",
            "output_path": str(output_path),
            "file_size": file_size,
            "relatorio_conformidade": relatorio_conformidade
        }

    except Exception as e:
        return {
            "status": "error",
            "error": f"Erro ao baixar PDF: {str(e)}"
        }


def main():
    """Exemplo de uso"""
    import sys

    if len(sys.argv) < 2:
        print("Uso: python download_pdf_example.py <arquivo.pdf> [idioma]")
        print("Idiomas disponíveis: pt-br (padrão), en, es")
        sys.exit(1)

    pdf_path = sys.argv[1]
    language = sys.argv[2] if len(sys.argv) > 2 else "pt-br"

    resultado = validar_e_baixar_relatorio_pdf(pdf_path, language=language, verbose=True)

    if resultado["status"] == "success":
        print(f"\n✓ Download concluído: {resultado['output_path']}")
    else:
        print(f"\n✗ Erro: {resultado.get('error', 'Erro desconhecido')}")
        if 'details' in resultado:
            print(f"Detalhes: {resultado['details']}")


if __name__ == "__main__":
    main()
