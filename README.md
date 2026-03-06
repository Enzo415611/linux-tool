<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:1e1e1e&height=120&section=header&text=Linux%20Tool%20🛠️&fontSize=40&fontColor=ffffff" alt="Banner Linux Tool" />
</p>

<h1 align="center">🛠️ Linux Tool</h1>

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Slint](https://img.shields.io/badge/Slint-2379F4?style=for-the-badge&logo=slint&logoColor=white)
![License: GPLv3](https://img.shields.io/badge/License-GPLv3-blue?style=for-the-badge)
![GitHub last commit](https://img.shields.io/github/last-commit/Enzo415611/linux-tool?style=for-the-badge)

</div>

O **linux-tool** é uma GUI minimalista e leve para gerenciar pacotes no **Arch Linux** e derivados.  
Suporta **pacotes oficiais** (via `pacman`) **e** **AUR** (via `yay`).  
Feito com **Slint** para uma interface declarativa nativa, performática e sem dependências pesadas.

disponível diretamente no AUR como **linux-tool-bin** para instalação em um comando só! 🚀

---

### ✨ Funcionalidades

- 🔍 **Busca em tempo real** — Para pacotes oficiais (pacman) e AUR (API RPC)
- 🎨 **Interface reativa e fluida** — 100% com Slint
- 🧠 **Caching inteligente** — Evita requisições repetidas à API do AUR
- 💾 **Configurações persistentes** — Salvas em `~/.config/linux-tool`

---

### 🛠️ Pré-requisitos

- **Rust** + **Cargo** (para compilar do fonte, opcional)
- **yay** — Para pacotes AUR (recomendado)
- **pacman** — Já incluso no Arch
- Sistema baseado em Arch Linux ou derivado

---

### 🚀 Instalação e Uso

#### Opção 1: Via AUR (mais fácil para usuários Arch – Recomendado!)

Use seu helper favorito (yay, paru, etc.):

```bash
yay -S linux-tool-bin
# ou
paru -S linux-tool-bin
```
Após instalar, rode:
```linux-tool```

Opção 2: Baixar binário pré-compilado manualmente

Acesse Releases:
👉 https://github.com/Enzo415611/linux-tool/releases
Baixe linux-tool (ou o .tar.gz equivalente).
Torne executável e execute: ```chmod +x linux-tool
./linux-tool```

Opção 3: Compilar do fonte
```Bash
git clone https://github.com/Enzo415611/linux-tool.git
cd linux-tool
cargo run --release
```
Ou build otimizado:
```bash
cargo build --release
# Binário em: target/release/linux-tool
```
Como usar:

Digite o nome do pacote na barra de busca (ex: "firefox", "neofetch", "visual-studio-code-bin").
A lista atualiza automaticamente (debounce de 700ms para AUR).
Pacotes oficiais: origem "pacman" / repositórios oficiais.
Pacotes AUR: origem "AUR".
Selecione → veja detalhes (versão, mantenedor, descrição).
Clique em Instalar → terminal abre com o comando correto (pacman -S ou yay -S).


  <img width="800" alt="Tela principal de busca" src="https://github.com/user-attachments/assets/9a87a46f-7b75-4dc5-849e-f8ee12ff8420">
  <img width="800" alt="Detalhes do pacote e instalação" src="https://github.com/user-attachments/assets/75cd0acb-5ee7-4a59-b6c3-e29472d6198c">
  


  Interface em ação: busca e detalhes de pacotes


⚖️ Licença
GNU General Public License v3.0 (GPL-3.0)
Código livre para uso, modificação e distribuição — mantendo a mesma licença e fonte aberto.
Veja LICENSE para detalhes.
