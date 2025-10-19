# 📦 Guide d'installation complet

Ce guide vous accompagne étape par étape pour installer et configurer le bot Discord Music en Rust.

## 📋 Table des matières

1. [Prérequis](#prérequis)
2. [Installation de Rust](#installation-de-rust)
3. [Installation de FFmpeg](#installation-de-ffmpeg)
4. [Installation de yt-dlp](#installation-de-yt-dlp)
5. [Configuration du bot Discord](#configuration-du-bot-discord)
6. [Compilation et lancement](#compilation-et-lancement)
7. [Dépannage](#dépannage)

---

## Prérequis

Avant de commencer, assurez-vous d'avoir:
- Un ordinateur sous Linux, macOS ou Windows
- Une connexion internet
- Un compte Discord
- Des droits d'administrateur sur votre machine

---

## Installation de Rust

### Linux / macOS

Ouvrez un terminal et exécutez:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Suivez les instructions à l'écran, puis rechargez votre shell:

```bash
source $HOME/.cargo/env
```

Vérifiez l'installation:

```bash
rustc --version
cargo --version
```

### Windows

1. Téléchargez [rustup-init.exe](https://rustup.rs/)
2. Exécutez le fichier et suivez les instructions
3. Redémarrez votre terminal
4. Vérifiez avec `rustc --version`

---

## Installation de FFmpeg

FFmpeg est nécessaire pour traiter l'audio.

### Ubuntu / Debian

```bash
sudo apt update
sudo apt install ffmpeg
```

### Fedora / CentOS

```bash
sudo dnf install ffmpeg
```

### macOS

```bash
brew install ffmpeg
```

### Windows

1. Téléchargez FFmpeg depuis [ffmpeg.org](https://ffmpeg.org/download.html)
2. Extrayez l'archive
3. Ajoutez le dossier `bin` au PATH système
4. Vérifiez avec `ffmpeg -version` dans CMD

---

## Installation de yt-dlp

yt-dlp permet de télécharger les musiques depuis YouTube.

### Linux / macOS

```bash
# Avec pip
pip3 install yt-dlp

# Ou avec votre gestionnaire de paquets
sudo apt install yt-dlp  # Ubuntu/Debian
brew install yt-dlp      # macOS
```

### Windows

```bash
# Avec pip
pip install yt-dlp

# Ou téléchargez l'exécutable depuis GitHub
# https://github.com/yt-dlp/yt-dlp/releases
```

Vérifiez l'installation:

```bash
yt-dlp --version
```

---

## Configuration du bot Discord

### 1. Créer une application Discord

1. Allez sur [Discord Developer Portal](https://discord.com/developers/applications)
2. Cliquez sur **New Application**
3. Donnez un nom à votre bot (ex: "Music Bot")
4. Cliquez sur **Create**

### 2. Créer le bot

1. Dans le menu de gauche, cliquez sur **Bot**
2. Cliquez sur **Add Bot** → **Yes, do it!**
3. Sous **TOKEN**, cliquez sur **Reset Token** puis **Copy**
4. **⚠️ Gardez ce token secret!**

### 3. Configurer les intents

Dans la section **Privileged Gateway Intents**:
- ✅ Activez **MESSAGE CONTENT INTENT**
- ✅ Activez **SERVER MEMBERS INTENT** (optionnel)

Cliquez sur **Save Changes**

### 4. Inviter le bot sur votre serveur

1. Dans le menu de gauche, cliquez sur **OAuth2** → **URL Generator**
2. Dans **SCOPES**, cochez:
   - `bot`
   - `applications.commands`
3. Dans **BOT PERMISSIONS**, cochez:
   - Read Messages/View Channels
   - Send Messages
   - Embed Links
   - Attach Files
   - Connect
   - Speak
   - Use Voice Activity
4. Copiez l'URL générée en bas
5. Ouvrez l'URL dans votre navigateur
6. Sélectionnez votre serveur Discord
7. Cliquez sur **Autoriser**

---

## Compilation et lancement

### 1. Cloner le projet

```bash
git clone https://github.com/LHRICO78/discord-music-bot-rust.git
cd discord-music-bot-rust
```

### 2. Configurer le token

Copiez le fichier d'exemple:

```bash
cp .env.example .env
```

Éditez `.env` avec votre éditeur préféré:

```bash
nano .env
# ou
code .env
# ou
vim .env
```

Remplacez `votre_token_discord_ici` par votre vrai token.

### 3. Compiler le bot

**Mode développement (plus rapide à compiler):**

```bash
cargo build
```

**Mode production (optimisé):**

```bash
cargo build --release
```

### 4. Lancer le bot

**Mode développement:**

```bash
cargo run
```

**Mode production:**

```bash
./target/release/discord-music-bot
```

### 5. Vérifier que ça fonctionne

Si tout va bien, vous devriez voir:

```
✅ VotreBot est connecté et prêt!
Bot ID: 123456789012345678
```

Sur Discord, le bot apparaît en ligne ✅

---

## Dépannage

### Le bot ne démarre pas

**Erreur: "DISCORD_BOT_TOKEN n'est pas définie"**

Solution: Vérifiez que le fichier `.env` existe et contient votre token.

```bash
cat .env
```

**Erreur: "error: linker `cc` not found"**

Solution: Installez les outils de compilation:

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/CentOS
sudo dnf groupinstall "Development Tools"

# macOS
xcode-select --install
```

### Le bot ne joue pas de musique

**Erreur: "ffmpeg: command not found"**

Solution: Installez FFmpeg (voir section ci-dessus)

**Erreur: "yt-dlp: command not found"**

Solution: Installez yt-dlp (voir section ci-dessus)

### Le bot se connecte mais ne répond pas

**Problème: Les commandes ne fonctionnent pas**

Solution: Vérifiez que vous avez activé **MESSAGE CONTENT INTENT** dans le Developer Portal.

### Erreur de compilation

**Erreur: "failed to compile"**

Solution: Mettez à jour Rust:

```bash
rustup update
```

Nettoyez et recompilez:

```bash
cargo clean
cargo build --release
```

### Le bot crash immédiatement

**Solution 1:** Vérifiez les logs:

```bash
RUST_LOG=debug cargo run
```

**Solution 2:** Vérifiez que le token est valide:
- Allez dans le Developer Portal
- Régénérez un nouveau token si nécessaire

---

## 🎉 Félicitations!

Votre bot est maintenant installé et fonctionnel!

### Prochaines étapes

1. Testez les commandes sur Discord: `!join`, `!play`, etc.
2. Consultez le [README.md](README.md) pour la liste complète des commandes
3. Configurez le démarrage automatique (voir section Déploiement du README)

### Besoin d'aide?

- Ouvrez une issue sur [GitHub](https://github.com/LHRICO78/discord-music-bot-rust/issues)
- Consultez la [documentation Serenity](https://docs.rs/serenity/)
- Rejoignez le serveur Discord de support (si disponible)

---

**Bon usage de votre bot! 🎵**

