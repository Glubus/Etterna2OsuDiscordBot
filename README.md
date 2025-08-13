# Bot Discord en Rust

Un bot Discord moderne créé avec Serenity en Rust, utilisant les **slash commands** (commandes slash) de Discord.

## 🏗️ Architecture Moderne

Le projet utilise les slash commands de Discord avec une architecture modulaire :

```
src/
├── main.rs              # Point d'entrée principal
├── bot.rs               # Logique principale du bot avec event handlers
├── calculator.rs        # Moteur de calcul mathématique
├── config.rs           # Gestion de la configuration
└── commands/           # Module des slash commands
    ├── mod.rs          # Définition des modules de commandes
    ├── ping.rs         # Commande /ping
    ├── calc.rs         # Commande /calc
    └── help.rs         # Commande /help
```

## Configuration

1. **Créer un bot Discord** :
   - Allez sur [Discord Developer Portal](https://discord.com/developers/applications)
   - Créez une nouvelle application
   - Dans l'onglet "Bot", créez un bot
   - Copiez le token du bot

2. **Configurer le token** :
   - Créez un fichier `.env` à la racine du projet
   - Ajoutez votre token : `DISCORD_TOKEN=votre_bot_token_ici`

3. **Inviter le bot sur votre serveur** :
   - Dans l'onglet "OAuth2" > "URL Generator"
   - Sélectionnez "bot" dans les scopes
   - Sélectionnez les permissions nécessaires (Send Messages, Use Slash Commands)
   - Utilisez l'URL générée pour inviter le bot

## Installation et exécution

```bash
# Installer les dépendances
cargo build

# Lancer le bot
cargo run
```

## Commandes disponibles

- `/calc <expression>` : Calcule une expression mathématique
  - Exemples : `/calc 2 + 2`, `/calc 10 * 5`, `/calc (3 + 4) * 2`
  - Opérateurs supportés : `+`, `-`, `*`, `/`, `(`, `)`

- `/ping` : Test de latence du bot

- `/help` : Affiche l'aide avec toutes les commandes

## Fonctionnalités

- ✅ **Slash commands modernes** - Utilise l'API moderne de Discord
- ✅ **Architecture modulaire** - Chaque commande dans son propre fichier
- ✅ **Connexion sécurisée** - Avec token Discord
- ✅ **Commande de calculatrice** - Parser d'expressions mathématiques
- ✅ **Validation et nettoyage** - Sécurité des entrées
- ✅ **Gestion d'erreurs robuste** - Messages d'erreur informatifs
- ✅ **Configuration flexible** - Variables d'environnement
- ✅ **Logs informatifs** - Avec emojis pour la lisibilité

## Structure des modules

### `config.rs`
- Gestion des variables d'environnement
- Configuration du bot (token)

### `bot.rs`
- Logique principale du bot avec event handlers
- Gestion des interactions slash commands
- Enregistrement automatique des commandes

### `commands/`
- **`mod.rs`** - Définition des modules de commandes
- **`ping.rs`** - Commande de test de latence
- **`calc.rs`** - Commande de calculatrice avec paramètres
- **`help.rs`** - Commande d'aide

### `calculator.rs`
- Moteur de calcul mathématique
- Parser d'expressions
- Évaluation sécurisée des expressions

## Avantages des Slash Commands

- ✅ **Interface native Discord** - Intégration parfaite avec Discord
- ✅ **Auto-complétion** - Suggestions automatiques
- ✅ **Validation des paramètres** - Discord gère la validation
- ✅ **Permissions intégrées** - Gestion native des permissions
- ✅ **Meilleure UX** - Interface utilisateur moderne
- ✅ **Pas besoin d'intents** - Plus simple à configurer
