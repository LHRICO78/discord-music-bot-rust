# 📊 Guide des profils de compilation

L'egg Pterodactyl permet maintenant de **choisir le profil de compilation** selon vos besoins et ressources disponibles.

## 🎯 Profils disponibles

### 1. 🏆 Profil **RELEASE** (recommandé)

**Quand l'utiliser :** Pour la production, serveurs avec au moins 2 GB de RAM

```
Paramètres:
- opt-level: 2
- lto: "thin"
- codegen-units: 16
```

**Caractéristiques :**
- ✅ **RAM nécessaire :** ~1.5 GB
- ✅ **Temps de compilation :** 5-7 minutes
- ✅ **Performance runtime :** 95%
- ✅ **Taille binaire :** ~18 MB
- ✅ **Consommation CPU (idle) :** <0.5%
- ✅ **Consommation RAM (runtime) :** ~40 MB

**Avantages :**
- Excellent équilibre performance/compilation
- Consommation mémoire réduite en runtime
- Démarrage ultra-rapide (~0.1s)
- Idéal pour serveurs de production

**Inconvénients :**
- Nécessite 1.5-2 GB de RAM pour compiler
- Compilation plus longue que "fast"

---

### 2. ⚡ Profil **FAST** (compilation rapide)

**Quand l'utiliser :** Serveurs avec RAM limitée (<2 GB), développement rapide

```
Paramètres:
- opt-level: 1
- lto: false
- codegen-units: 256
```

**Caractéristiques :**
- ✅ **RAM nécessaire :** ~800 MB
- ✅ **Temps de compilation :** 2-3 minutes
- ⚠️ **Performance runtime :** 85%
- ⚠️ **Taille binaire :** ~25 MB
- ⚠️ **Consommation CPU (idle) :** ~1%
- ⚠️ **Consommation RAM (runtime) :** ~60 MB

**Avantages :**
- Compilation très rapide
- Nécessite peu de RAM
- Parfait pour tests et développement
- Fonctionne sur serveurs avec RAM limitée

**Inconvénients :**
- Performance légèrement réduite (15%)
- Binaire plus gros
- Consomme plus de ressources en runtime

---

### 3. 🐛 Profil **DEBUG** (développement)

**Quand l'utiliser :** Développement, débogage, tests uniquement

```
Paramètres:
- opt-level: 0
- lto: false
- debug: true
```

**Caractéristiques :**
- ✅ **RAM nécessaire :** ~500 MB
- ✅ **Temps de compilation :** 1-2 minutes
- ❌ **Performance runtime :** 30-40%
- ❌ **Taille binaire :** ~100 MB
- ❌ **Consommation CPU (idle) :** ~3-5%
- ❌ **Consommation RAM (runtime) :** ~150 MB

**Avantages :**
- Compilation ultra-rapide
- Très peu de RAM nécessaire
- Inclut les symboles de debug
- Idéal pour développement local

**Inconvénients :**
- ⚠️ **TRÈS LENT** en runtime (2-3x plus lent)
- Binaire énorme
- Consomme beaucoup de ressources
- **NE PAS UTILISER EN PRODUCTION**

---

## 📋 Tableau comparatif

| Critère | Release | Fast | Debug |
|---------|---------|------|-------|
| **RAM compilation** | 1.5 GB | 800 MB | 500 MB |
| **Temps compilation** | 5-7 min | 2-3 min | 1-2 min |
| **Performance runtime** | 95% | 85% | 30-40% |
| **Taille binaire** | 18 MB | 25 MB | 100 MB |
| **RAM runtime** | 40 MB | 60 MB | 150 MB |
| **CPU idle** | <0.5% | ~1% | 3-5% |
| **Démarrage** | 0.1s | 0.2s | 1-2s |
| **Production** | ✅ Recommandé | ⚠️ Acceptable | ❌ Non |
| **Développement** | ⚠️ Lent à compiler | ✅ Bon compromis | ✅ Idéal |

---

## 🔧 Comment changer de profil dans Pterodactyl

### Méthode 1 : Via l'interface Pterodactyl

1. Allez dans votre serveur Discord Bot
2. Cliquez sur l'onglet **Startup**
3. Trouvez la variable **Build Profile**
4. Changez la valeur :
   - `release` → Pour production (recommandé)
   - `fast` → Pour compilation rapide
   - `debug` → Pour développement uniquement
5. **Réinstallez le serveur** (Settings → Reinstall Server)
6. Ou redémarrez si **Auto Update = 1**

### Méthode 2 : Lors de la création du serveur

Lors de la création d'un nouveau serveur avec l'egg :
1. Remplissez les informations du serveur
2. Dans les variables, définissez **BUILD_PROFILE**
3. Choisissez selon vos ressources disponibles

---

## 💡 Recommandations par scénario

### Serveur de production (24/7)
```
Profil: release
RAM allouée: 2 GB minimum
Raison: Meilleures performances, faible consommation
```

### Serveur de test/développement
```
Profil: fast
RAM allouée: 1 GB minimum
Raison: Compilation rapide, performances acceptables
```

### Machine avec RAM très limitée (<1 GB)
```
Profil: fast
RAM allouée: 1 GB
Raison: Seul profil qui compile avec <1 GB
```

### Développement local avec recompilations fréquentes
```
Profil: debug ou fast
RAM allouée: 1 GB
Raison: Compilation ultra-rapide
```

### Serveur Discord très actif (100+ utilisateurs)
```
Profil: release
RAM allouée: 4 GB
Raison: Performances maximales nécessaires
```

---

## 🚨 Résolution de problèmes

### La compilation échoue avec "release"

**Symptôme :** Signal 9 (SIGKILL), out of memory

**Solution :**
1. Passez au profil **fast**
2. Ou augmentez la RAM allouée à 2+ GB
3. Ou activez le swap sur votre système

### Le bot est trop lent en runtime

**Symptôme :** Latence élevée, commandes lentes

**Solution :**
1. Vérifiez que vous n'utilisez pas le profil **debug**
2. Passez au profil **release** si vous êtes en **fast**
3. Augmentez les ressources CPU/RAM du serveur

### La compilation prend trop de temps

**Symptôme :** Plus de 10 minutes de compilation

**Solution :**
1. Passez au profil **fast** (2-3 min au lieu de 5-7 min)
2. Augmentez les ressources CPU du serveur
3. Vérifiez que vous n'avez pas de limite I/O disque

### Le binaire est trop gros

**Symptôme :** Binaire de 100+ MB

**Solution :**
1. Vous utilisez probablement le profil **debug**
2. Passez au profil **release** (18 MB) ou **fast** (25 MB)
3. Le profil debug inclut tous les symboles de débogage

---

## 📈 Optimisations avancées

### Modifier les profils dans Cargo.toml

Si vous voulez personnaliser davantage, éditez le fichier `Cargo.toml` :

```toml
[profile.release]
opt-level = 2           # 0-3, plus élevé = plus rapide mais plus de RAM
lto = "thin"            # false, "thin", ou true
codegen-units = 16      # 1-256, plus élevé = moins de RAM
strip = true            # Supprime les symboles debug
```

### Créer un profil personnalisé

Ajoutez dans `Cargo.toml` :

```toml
[profile.custom]
inherits = "release"
opt-level = 2
lto = "thin"
codegen-units = 32      # Ajustez selon vos besoins
```

Puis compilez avec :
```bash
cargo build --profile custom
```

---

## 🎓 Comprendre les paramètres

### opt-level (niveau d'optimisation)
- `0` : Aucune optimisation (debug)
- `1` : Optimisations basiques
- `2` : Optimisations standard (bon équilibre)
- `3` : Optimisations maximales (plus de RAM)

### lto (Link-Time Optimization)
- `false` : Pas d'optimisation au linkage (rapide, peu de RAM)
- `"thin"` : Optimisation légère (bon compromis)
- `true` : Optimisation complète (lent, beaucoup de RAM)

### codegen-units (parallélisation)
- `1` : Tout en un seul thread (lent, beaucoup de RAM)
- `16` : Bon équilibre
- `256` : Maximum de parallélisation (rapide, peu de RAM)

---

## ✅ Résumé

**Pour la plupart des utilisateurs :**
- Utilisez **release** si vous avez 2+ GB de RAM
- Utilisez **fast** si vous avez <2 GB de RAM
- N'utilisez **jamais debug** en production

**L'egg Pterodactyl gère automatiquement tout !**
Il suffit de choisir le profil dans l'onglet Startup. 🚀

