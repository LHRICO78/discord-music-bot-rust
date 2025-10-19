# 🦖 Installation sur Pterodactyl Panel (Rust)

Ce guide vous explique comment installer et configurer le bot Discord Music **Rust** sur un panel Pterodactyl.

## 📋 Prérequis

- Accès à un panel Pterodactyl (admin ou utilisateur)
- Un token de bot Discord
- Le fichier egg: `egg-discord-music-bot-rust.json`

---

## 🎯 Installation pour les administrateurs

### Étape 1: Importer l'egg

1. Connectez-vous à votre **Panel Admin Pterodactyl**
2. Allez dans **Nests** (Nids)
3. Sélectionnez ou créez un nest (par exemple "Discord Bots")
4. Cliquez sur **Import Egg** (Importer un Egg)
5. Uploadez le fichier `egg-discord-music-bot-rust.json`
6. Cliquez sur **Import**

### Étape 2: Configurer l'egg (optionnel)

Après l'import, vous pouvez modifier:
- Le nom de l'egg
- La description
- L'image Docker (par défaut: `ghcr.io/parkervcp/yolks:debian`)
- Les variables d'environnement

### Étape 3: Créer un serveur

1. Allez dans **Servers** (Serveurs)
2. Cliquez sur **Create New** (Créer nouveau)
3. Remplissez les informations:
   - **Name**: Discord Music Bot Rust
   - **Owner**: Sélectionnez l'utilisateur
   - **Nest**: Sélectionnez le nest où vous avez importé l'egg
   - **Egg**: Discord Music Bot (Rust)
   - **Docker Image**: `ghcr.io/parkervcp/yolks:debian`

4. **Allocation**:
   - Assignez une IP et un port (pas utilisé par le bot, mais requis par Pterodactyl)

5. **Resource Limits** (recommandé pour Rust):
   - **Memory**: 1024 MB minimum (2048 MB recommandé pour la compilation)
   - **Disk**: 2048 MB minimum (4096 MB recommandé)
   - **CPU**: 200% minimum (pour la compilation)

6. Cliquez sur **Create Server**

---

## 👤 Installation pour les utilisateurs

### Étape 1: Accéder à votre serveur

1. Connectez-vous au **Panel Pterodactyl**
2. Sélectionnez votre serveur Discord Music Bot Rust

### Étape 2: Configuration initiale

1. Allez dans l'onglet **Startup**
2. Configurez les variables:

#### **Variables obligatoires:**

**Discord Bot Token**
- Votre token Discord
- Obtenez-le sur https://discord.com/developers/applications
- Exemple: `VOTRE_TOKEN_DISCORD_ICI`

#### **Variables optionnelles:**

**Git Repository Address**
- Adresse du dépôt GitHub (sans `https://`)
- Par défaut: `github.com/LHRICO78/discord-music-bot-rust.git`
- Modifiez si vous utilisez votre propre fork

**Auto Update**
- `1` = Mise à jour automatique depuis GitHub au démarrage
- `0` = Pas de mise à jour automatique
- Par défaut: `1` (recommandé)

**Rust Log Level**
- Niveau de log: `trace`, `debug`, `info`, `warn`, `error`
- Par défaut: `info`
- Utilisez `debug` pour plus de détails en cas de problème

**User Upload**
- `false` = Cloner depuis GitHub (recommandé)
- `true` = Utiliser les fichiers uploadés manuellement
- Par défaut: `false`

**Git Username** et **Git Access Token**
- Nécessaires uniquement pour les dépôts privés
- Laissez vide pour les dépôts publics

### Étape 3: Installation

1. Allez dans l'onglet **Console**
2. Cliquez sur **Install** ou **Reinstall** si déjà installé
3. Attendez la fin de l'installation et de la **compilation** (cela peut prendre 5-10 minutes)
4. Vous verrez "Installation complete!" et "Compilation completed successfully!"

⚠️ **Important:** La première compilation Rust prend du temps (5-10 minutes). C'est normal!

### Étape 4: Démarrer le bot

1. Cliquez sur **Start** (Démarrer)
2. Attendez que le bot se connecte
3. Vous devriez voir dans la console:
   ```
   ✅ VotreBot est connecté et prêt!
   Bot ID: 123456789012345678
   ```

---

## 🎮 Utilisation

Une fois le bot démarré, il est prêt à être utilisé sur Discord!

### Commandes disponibles

- `!join` - Rejoindre le salon vocal
- `!play <url ou recherche>` - Jouer une musique
- `!pause` - Mettre en pause
- `!resume` - Reprendre
- `!skip` - Passer à la suivante
- `!stop` - Arrêter et vider la file
- `!queue` - Voir la file d'attente
- `!volume <0-100>` - Changer le volume
- `!nowplaying` ou `!np` - Afficher la musique en cours
- `!leave` - Quitter le salon vocal

---

## 🔧 Gestion du serveur

### Démarrer le bot
Cliquez sur **Start** dans la console

### Arrêter le bot
Cliquez sur **Stop** dans la console

### Redémarrer le bot
Cliquez sur **Restart** dans la console

### Voir les logs
Les logs s'affichent en temps réel dans la **Console**

### Mettre à jour le bot

**Si Auto Update = 1:**
- Redémarrez simplement le bot
- Il se mettra à jour automatiquement depuis GitHub
- La recompilation prendra quelques minutes

**Si Auto Update = 0:**
1. Allez dans **Files** (Fichiers)
2. Supprimez tous les fichiers
3. Allez dans **Settings** (Paramètres)
4. Cliquez sur **Reinstall Server**

---

## 📁 Gestion des fichiers

### Accéder aux fichiers

1. Allez dans l'onglet **Files**
2. Vous verrez tous les fichiers du bot:
   - `src/main.rs` - Code principal du bot
   - `Cargo.toml` - Configuration et dépendances
   - `target/release/discord-music-bot` - Binaire compilé
   - `README.md` - Documentation
   - Etc.

### Modifier le code

1. Cliquez sur `src/main.rs`
2. Modifiez le code
3. Cliquez sur **Save Content**
4. **Recompilez** en redémarrant le bot

⚠️ **Note:** Toute modification du code nécessite une recompilation (5-10 minutes)

---

## ⚙️ Configuration avancée

### Changer l'image Docker

**Images disponibles:**
- `ghcr.io/parkervcp/yolks:debian` (recommandé)
- `ghcr.io/parkervcp/yolks:ubuntu`

**Note:** Rust est installé automatiquement lors de l'installation

**Pour changer:**
1. Arrêtez le serveur
2. Allez dans **Startup**
3. Changez **Docker Image**
4. Redémarrez le serveur

### Augmenter les ressources

Si la compilation échoue ou est trop lente:

1. Contactez votre administrateur Pterodactyl
2. Demandez une augmentation de:
   - **Memory**: 2048 MB ou plus (important pour la compilation)
   - **CPU**: 200% ou plus (accélère la compilation)
   - **Disk**: 4096 MB ou plus

### Utiliser un fork personnel

1. Forkez le dépôt GitHub
2. Dans **Startup**, changez **Git Repository Address**:
   - Exemple: `github.com/votre-username/discord-music-bot-rust.git`
3. Réinstallez le serveur

---

## 🐛 Dépannage

### Le bot ne démarre pas

**Vérifier le token Discord:**
1. Allez dans **Startup**
2. Vérifiez que **Discord Bot Token** est correct
3. Régénérez un nouveau token si nécessaire sur https://discord.com/developers/applications

**Vérifier les logs:**
1. Allez dans **Console**
2. Lisez les messages d'erreur
3. Erreurs courantes:
   - `DISCORD_BOT_TOKEN n'est pas définie` → Token manquant
   - `Improper token` → Token invalide
   - `compilation failed` → Problème de compilation

**Réinstaller:**
1. Allez dans **Settings**
2. Cliquez sur **Reinstall Server**
3. Attendez la fin de l'installation et de la compilation
4. Redémarrez

### La compilation échoue

**Erreur: "out of memory"**

Solution: Augmentez la RAM allouée au serveur (minimum 2048 MB)

**Erreur: "compilation timeout"**

Solution: Augmentez le CPU alloué ou attendez plus longtemps

**Erreur: "linker error"**

Solution: Réinstallez le serveur (les dépendances seront réinstallées)

### Le bot crash immédiatement

**Vérifier la mémoire:**
- Le bot Rust nécessite au moins 512 MB de RAM en exécution
- La compilation nécessite au moins 1024 MB

**Vérifier FFmpeg:**
- FFmpeg est installé automatiquement au premier démarrage
- Si ça échoue, réinstallez le serveur

### Le bot ne joue pas de musique

**Vérifier FFmpeg:**
1. Allez dans **Console**
2. Tapez: `ffmpeg -version`
3. Si "command not found", réinstallez le serveur

**Vérifier yt-dlp:**
1. Tapez: `yt-dlp --version`
2. Si "command not found", réinstallez le serveur

**Vérifier les permissions Discord:**
- Le bot doit avoir les permissions:
  - Connect (Se connecter)
  - Speak (Parler)
  - Use Voice Activity (Utiliser la détection de voix)

---

## 📊 Ressources recommandées

### Configuration minimale
- **Memory**: 1024 MB (512 MB runtime + 512 MB compilation)
- **Disk**: 2048 MB
- **CPU**: 100%

### Configuration recommandée
- **Memory**: 2048 MB (compilation plus rapide)
- **Disk**: 4096 MB
- **CPU**: 200%

### Configuration optimale (serveurs actifs)
- **Memory**: 4096 MB
- **Disk**: 8192 MB
- **CPU**: 300%

---

## 🔒 Sécurité

### Protéger votre token Discord

⚠️ **IMPORTANT**: Ne partagez jamais votre token Discord!

**Si votre token est compromis:**
1. Allez sur https://discord.com/developers/applications
2. Sélectionnez votre application
3. Allez dans **Bot**
4. Cliquez sur **Reset Token**
5. Copiez le nouveau token
6. Mettez-le à jour dans **Startup** sur Pterodactyl
7. Redémarrez le bot

---

## ⚡ Avantages de la version Rust

### Comparé à la version Python

| Aspect | Python | Rust |
|--------|--------|------|
| **Mémoire (runtime)** | ~150 MB | ~40 MB |
| **Démarrage** | ~2s | ~0.1s |
| **CPU (idle)** | 1-2% | <0.5% |
| **Performance** | Moyenne | Excellente |
| **Compilation** | ❌ Non | ✅ Oui (5-10 min) |
| **Mise à jour** | Instantanée | Recompilation nécessaire |

### Quand utiliser Rust ?

✅ **Utilisez Rust si:**
- Vous voulez les meilleures performances
- Vous avez un serveur avec peu de RAM
- Vous hébergez plusieurs bots
- Vous voulez un bot ultra-léger

⚠️ **Utilisez Python si:**
- Vous voulez des mises à jour instantanées
- Vous n'avez pas assez de RAM pour compiler (< 1 GB)
- Vous préférez la simplicité

---

## 🔄 Mises à jour

### Mise à jour automatique (Auto Update = 1)

Le bot se met à jour automatiquement depuis GitHub à chaque redémarrage:
1. Redémarrez simplement le bot
2. Il téléchargera les dernières modifications
3. Il recompilera le projet (5-10 minutes)

### Mise à jour manuelle

1. Allez dans **Files**
2. Supprimez tous les fichiers
3. Allez dans **Settings**
4. Cliquez sur **Reinstall Server**

---

## 📞 Support

### Problèmes avec le bot

Ouvrez une issue sur GitHub:
https://github.com/LHRICO78/discord-music-bot-rust/issues

### Problèmes avec Pterodactyl

Contactez votre administrateur Pterodactyl ou consultez:
https://pterodactyl.io/community/about.html

---

## 📝 Notes importantes

- ⚠️ La première compilation prend 5-10 minutes (c'est normal pour Rust)
- ⚠️ Chaque mise à jour nécessite une recompilation
- ✅ Une fois compilé, le bot est ultra-rapide et léger
- ✅ Le binaire compilé est dans `target/release/discord-music-bot`
- ✅ FFmpeg et yt-dlp sont installés automatiquement
- ✅ Le bot nécessite au moins 1 GB de RAM pour compiler

---

**Votre bot Discord Music Rust est maintenant prêt sur Pterodactyl ! 🦀**

Pour toute question, consultez la documentation complète sur GitHub.

