# 🛠️ Linux Tool

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Slint](https://img.shields.io/badge/Slint-23B14D?style=for-the-badge&logo=qt&logoColor=white)
![License](https://img.shields.io/badge/license-GPLv3-blue?style=for-the-badge)

O **linux-tool** é uma interface gráfica (GUI) minimalista projetada para facilitar a interação com o AUR (Arch User Repository). Ele utiliza o **Slint** para a interface do usuário e o **yay** como backend para gerenciamento de pacotes, permitindo pesquisar e instalar softwares de forma rápida e segura.

---

## 🚀 Funcionalidades

* **Busca em Tempo Real:** Integração direta com a API RPC do AUR para listagem instantânea de pacotes.
* **Interface Reativa:** Desenvolvida com Slint, oferecendo uma experiência visual fluida e leve.
* **Caching Inteligente:** Otimiza o desempenho evitando requisições repetitivas à API para a mesma pesquisa através de um estado de aplicação persistente.
* **Instalação Segura:** Utiliza `pkexec` (Polkit) para solicitar permissões de administrador (root) apenas no momento exato da instalação.

---

## 🛠️ Pré-requisitos

Para compilar e rodar este projeto, certifique-se de ter as seguintes ferramentas instaladas no seu sistema:

1.  **Rust & Cargo**: [Instalação oficial](https://www.rust-lang.org/tools/install).
2.  **Yay**: O projeto assume que o `yay` está instalado para gerenciar o AUR.
---

## 🔧 Instalação e Uso

### 1. Instalação

Clone o repositório e entre na pasta do projeto:

```bash
git clone https://github.com/Enzo415611/linux-tool.git
cd linux-tool
```
Compile e execute o projeto em modo release (otimizado):
```bash

cargo run --release
```
### 2. Como usar

    Digite o nome do pacote desejado na barra de busca superior.

    A lista será atualizada (existe um debounce de 700ms para evitar sobrecarga na API).

    Selecione um pacote na lista para visualizar os detalhes (versão, mantenedor, descrição).

    Clique no botão Instalar para iniciar o processo via terminal.



<img width="1920" height="1041" alt="image" src="https://github.com/user-attachments/assets/d53c2a7e-8e49-444d-b986-fa6648553e67" />

<img width="1920" height="1041" alt="image" src="https://github.com/user-attachments/assets/d8af49e1-2ff5-45d5-ba41-6606ee50485b" />

### ⚖️ Licença

Este projeto está licenciado sob a GNU General Public License v3.0 (GPL-3.0).

Isso garante que o software permaneça livre para todos os usuários, permitindo cópia, modificação e distribuição, desde que as alterações também sejam licenciadas sob a GPL e o código-fonte permaneça aberto. Consulte o arquivo LICENSE para mais detalhes.
