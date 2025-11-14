"""
Interface Gráfica Tkinter para Validador de Assinaturas PDF - ITI
Permite validar assinaturas digitais em PDFs através de uma interface gráfica amigável.
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
from pathlib import Path
import threading
from validator_api import validate_pdf


class ValidadorGUI:
    """Interface gráfica para validação de assinaturas PDF."""
    
    def __init__(self, root):
        self.root = root
        self.root.title("Validador de Assinaturas PDF - ITI")
        self.root.geometry("800x600")
        self.root.resizable(True, True)
        
        # Variáveis
        self.arquivo_selecionado = tk.StringVar()
        self.validando = False
        
        # Configurar interface
        self.criar_widgets()
        
    def criar_widgets(self):
        """Cria todos os widgets da interface."""
        
        # Frame principal com padding
        main_frame = ttk.Frame(self.root, padding="10")
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        
        # Configurar grid para redimensionamento
        self.root.columnconfigure(0, weight=1)
        self.root.rowconfigure(0, weight=1)
        main_frame.columnconfigure(0, weight=1)
        main_frame.rowconfigure(3, weight=1)
        
        # Título
        titulo = ttk.Label(
            main_frame,
            text="Validador de Assinaturas Digitais PDF",
            font=("Arial", 16, "bold")
        )
        titulo.grid(row=0, column=0, columnspan=3, pady=(0, 20))
        
        # Frame de seleção de arquivo
        arquivo_frame = ttk.LabelFrame(main_frame, text="Arquivo PDF", padding="10")
        arquivo_frame.grid(row=1, column=0, columnspan=3, sticky=(tk.W, tk.E), pady=(0, 10))
        arquivo_frame.columnconfigure(1, weight=1)
        
        # Label do arquivo
        ttk.Label(arquivo_frame, text="Arquivo:").grid(row=0, column=0, sticky=tk.W, padx=(0, 5))
        
        # Entry do caminho do arquivo
        self.entry_arquivo = ttk.Entry(arquivo_frame, textvariable=self.arquivo_selecionado, state="readonly")
        self.entry_arquivo.grid(row=0, column=1, sticky=(tk.W, tk.E), padx=(0, 5))
        
        # Botão selecionar arquivo
        self.btn_selecionar = ttk.Button(
            arquivo_frame,
            text="Selecionar PDF",
            command=self.selecionar_arquivo
        )
        self.btn_selecionar.grid(row=0, column=2)
        
        # Frame de ações
        acoes_frame = ttk.Frame(main_frame)
        acoes_frame.grid(row=2, column=0, columnspan=3, pady=(0, 10))
        
        # Botão validar
        self.btn_validar = ttk.Button(
            acoes_frame,
            text="Validar Assinatura",
            command=self.validar_arquivo,
            state="disabled"
        )
        self.btn_validar.grid(row=0, column=0, padx=5)
        
        # Botão limpar
        self.btn_limpar = ttk.Button(
            acoes_frame,
            text="Limpar",
            command=self.limpar_resultados
        )
        self.btn_limpar.grid(row=0, column=1, padx=5)
        
        # Progress bar
        self.progress = ttk.Progressbar(acoes_frame, mode='indeterminate', length=200)
        self.progress.grid(row=0, column=2, padx=5)
        
        # Frame de resultados
        resultados_frame = ttk.LabelFrame(main_frame, text="Resultados", padding="10")
        resultados_frame.grid(row=3, column=0, columnspan=3, sticky=(tk.W, tk.E, tk.N, tk.S))
        resultados_frame.columnconfigure(0, weight=1)
        resultados_frame.rowconfigure(0, weight=1)
        
        # Área de texto para resultados com scroll
        self.texto_resultados = scrolledtext.ScrolledText(
            resultados_frame,
            wrap=tk.WORD,
            width=80,
            height=20,
            font=("Courier", 10)
        )
        self.texto_resultados.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        
        # Adicionar mensagem inicial
        self.mostrar_mensagem_inicial()
        
        # Rodapé com informações
        rodape = ttk.Label(
            main_frame,
            text="Validador de Assinaturas PDF - ITI | Desenvolvido com Python",
            font=("Arial", 8),
            foreground="gray"
        )
        rodape.grid(row=4, column=0, columnspan=3, pady=(10, 0))
        
    def mostrar_mensagem_inicial(self):
        """Mostra mensagem inicial na área de resultados."""
        mensagem = """╔══════════════════════════════════════════════════════════════════════════╗
║                  BEM-VINDO AO VALIDADOR DE ASSINATURAS PDF                ║
╚══════════════════════════════════════════════════════════════════════════╝

Como usar:

1. Clique em "Selecionar PDF" para escolher um arquivo PDF
2. Clique em "Validar Assinatura" para verificar as assinaturas digitais
3. Os resultados serão exibidos nesta área

Funcionalidades:
✓ Validação através da API oficial do ITI
✓ Informações detalhadas sobre assinaturas
✓ Dados de certificados digitais
✓ Status de validade

Aguardando seleção de arquivo...
"""
        self.texto_resultados.delete(1.0, tk.END)
        self.texto_resultados.insert(1.0, mensagem)
        
    def selecionar_arquivo(self):
        """Abre diálogo para selecionar arquivo PDF."""
        arquivo = filedialog.askopenfilename(
            title="Selecione um arquivo PDF",
            filetypes=[("Arquivos PDF", "*.pdf"), ("Todos os arquivos", "*.*")]
        )
        
        if arquivo:
            self.arquivo_selecionado.set(arquivo)
            self.btn_validar.config(state="normal")
            self.texto_resultados.delete(1.0, tk.END)
            self.texto_resultados.insert(1.0, f"✓ Arquivo selecionado: {Path(arquivo).name}\n\n")
            self.texto_resultados.insert(tk.END, "Clique em 'Validar Assinatura' para iniciar a validação.")
            
    def validar_arquivo(self):
        """Inicia a validação do arquivo em uma thread separada."""
        if not self.arquivo_selecionado.get():
            messagebox.showwarning("Aviso", "Selecione um arquivo PDF primeiro!")
            return
            
        # Desabilitar botões durante validação
        self.btn_validar.config(state="disabled")
        self.btn_selecionar.config(state="disabled")
        self.btn_limpar.config(state="disabled")
        
        # Iniciar progress bar
        self.progress.start(10)
        self.validando = True
        
        # Limpar resultados anteriores
        self.texto_resultados.delete(1.0, tk.END)
        self.texto_resultados.insert(1.0, "⏳ Validando arquivo...\n")
        self.texto_resultados.insert(tk.END, "📤 Enviando para API do ITI...\n\n")
        
        # Executar validação em thread separada
        thread = threading.Thread(target=self.executar_validacao)
        thread.daemon = True
        thread.start()
        
    def executar_validacao(self):
        """Executa a validação e atualiza a interface."""
        try:
            # Validar PDF
            resultado = validate_pdf(self.arquivo_selecionado.get(), verbose=False)
            
            # Atualizar interface na thread principal
            self.root.after(0, self.mostrar_resultado, resultado)
            
        except Exception as e:
            self.root.after(0, self.mostrar_erro, str(e))
        finally:
            # Parar progress bar e reabilitar botões
            self.root.after(0, self.finalizar_validacao)
            
    def mostrar_resultado(self, resultado):
        """Mostra o resultado da validação na interface."""
        self.texto_resultados.delete(1.0, tk.END)
        
        # Cabeçalho
        self.texto_resultados.insert(1.0, "═" * 76 + "\n")
        self.texto_resultados.insert(tk.END, " " * 20 + "RESULTADO DA VALIDAÇÃO\n")
        self.texto_resultados.insert(tk.END, "═" * 76 + "\n\n")
        
        status = resultado.get('status', 'unknown')
        
        if status == 'valid':
            # Documento válido
            self.texto_resultados.insert(tk.END, "✅ STATUS: VÁLIDO\n\n", "sucesso")
            
            # Informações do documento
            if 'documento' in resultado:
                doc = resultado['documento']
                self.texto_resultados.insert(tk.END, "📄 INFORMAÇÕES DO DOCUMENTO:\n")
                self.texto_resultados.insert(tk.END, "─" * 76 + "\n")
                self.texto_resultados.insert(tk.END, f"   Nome: {doc.get('nome_arquivo', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   Hash: {doc.get('hash', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   Data Validação: {doc.get('data_validacao', 'N/A')}\n\n")
            
            # Assinaturas
            total = resultado.get('total_assinaturas', 0)
            self.texto_resultados.insert(tk.END, f"✍️  ASSINATURAS ENCONTRADAS: {total}\n")
            self.texto_resultados.insert(tk.END, "─" * 76 + "\n\n")
            
            for i, assinatura in enumerate(resultado.get('assinaturas', []), 1):
                self.texto_resultados.insert(tk.END, f"[Assinatura {i}]\n")
                self.texto_resultados.insert(tk.END, f"   👤 Assinado por: {assinatura.get('assinado_por', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   🆔 CPF: {assinatura.get('cpf', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   🏢 Certificadora: {assinatura.get('certificadora', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   🔢 Nº Série: {assinatura.get('numero_serie_certificado', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   📅 Data: {assinatura.get('data_assinatura', 'N/A')}\n")
                self.texto_resultados.insert(tk.END, f"   ⚡ Status: {assinatura.get('status', 'N/A')}\n")
                carimbo = "Sim ✓" if assinatura.get('possui_carimbo_tempo', False) else "Não ✗"
                self.texto_resultados.insert(tk.END, f"   ⏱️  Carimbo de Tempo: {carimbo}\n\n")
                
        elif status == 'invalid':
            # Documento inválido
            self.texto_resultados.insert(tk.END, "❌ STATUS: INVÁLIDO\n\n", "erro")
            erro_msg = resultado.get('error', 'Documento sem assinatura ou inválido')
            self.texto_resultados.insert(tk.END, f"Motivo: {erro_msg}\n\n")
            
        else:
            # Erro
            self.texto_resultados.insert(tk.END, "⚠️  STATUS: ERRO\n\n", "aviso")
            erro_msg = resultado.get('error', 'Erro desconhecido durante a validação')
            self.texto_resultados.insert(tk.END, f"Erro: {erro_msg}\n\n")
        
        # Rodapé
        self.texto_resultados.insert(tk.END, "─" * 76 + "\n")
        self.texto_resultados.insert(tk.END, "Validação concluída.\n")
        
        # Configurar tags de cor
        self.texto_resultados.tag_config("sucesso", foreground="green", font=("Courier", 10, "bold"))
        self.texto_resultados.tag_config("erro", foreground="red", font=("Courier", 10, "bold"))
        self.texto_resultados.tag_config("aviso", foreground="orange", font=("Courier", 10, "bold"))
        
    def mostrar_erro(self, mensagem_erro):
        """Mostra mensagem de erro."""
        self.texto_resultados.delete(1.0, tk.END)
        self.texto_resultados.insert(1.0, "═" * 76 + "\n")
        self.texto_resultados.insert(tk.END, " " * 30 + "ERRO\n")
        self.texto_resultados.insert(tk.END, "═" * 76 + "\n\n")
        self.texto_resultados.insert(tk.END, "⚠️  Ocorreu um erro durante a validação:\n\n")
        self.texto_resultados.insert(tk.END, f"{mensagem_erro}\n\n")
        self.texto_resultados.insert(tk.END, "Verifique se:\n")
        self.texto_resultados.insert(tk.END, "  • O arquivo é um PDF válido\n")
        self.texto_resultados.insert(tk.END, "  • Você tem conexão com a internet\n")
        self.texto_resultados.insert(tk.END, "  • O arquivo não está corrompido\n")
        
    def finalizar_validacao(self):
        """Finaliza a validação e reabilita botões."""
        self.progress.stop()
        self.validando = False
        self.btn_validar.config(state="normal")
        self.btn_selecionar.config(state="normal")
        self.btn_limpar.config(state="normal")
        
    def limpar_resultados(self):
        """Limpa a seleção e resultados."""
        self.arquivo_selecionado.set("")
        self.btn_validar.config(state="disabled")
        self.mostrar_mensagem_inicial()


def main():
    """Função principal para iniciar a aplicação."""
    root = tk.Tk()
    app = ValidadorGUI(root)
    root.mainloop()


if __name__ == "__main__":
    main()
