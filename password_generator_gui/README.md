# Générateur de mots de passe GUI

Un générateur de mots de passe sécurisés et configurables écrit en Rust avec **interface graphique Slint** moderne et intuitive.

## 🚀 Fonctionnalités

- **Interface graphique moderne** : Interface utilisateur intuitive avec Slint
- **Longueur configurable** : Spécifiez la longueur exacte de votre mot de passe (4-128 caractères)
- **Types de caractères sélectionnables** : Contrôlez précisément quels types de caractères inclure
- **Exclusion de caractères** : Excluez des caractères spécifiques si nécessaire
- **Génération multiple** : Générez jusqu'à 20 mots de passe en une seule fois
- **Interface CLI alternative** : Utilise aussi clap pour une expérience en ligne de commande

## 📦 Installation

```bash
# Cloner le projet et naviguer dans le dossier
cd password_generator_gui

# Compiler le projet
cargo build --release

# Lancer l'interface graphique
cargo run
```

## 🖥️ Interface Graphique

L'application s'ouvre avec une interface graphique moderne comportant :

### Panneau de Configuration
- **Longueur du mot de passe** : Utilisez la boîte de sélection pour choisir entre 4 et 128 caractères
- **Nombre à générer** : Générez de 1 à 20 mots de passe simultanément
- **Types de caractères** : Cochez/décochez les options pour :
  - ✅ Minuscules (a-z)
  - ✅ Majuscules (A-Z)
  - ✅ Chiffres (0-9)
  - ✅ Symboles (!@#$...)
- **Caractères à exclure** : Tapez les caractères à éviter (ex: `0O1l` pour éviter la confusion)
- **Statistiques détaillées** : Cochez pour voir l'analyse de force

### Génération
- Cliquez sur **🎲 Générer les mots de passe** pour créer vos mots de passe
- Les résultats apparaissent dans la zone de texte en bas

### Gestion des Résultats
- **📋 Copier tout** : Prépare le texte pour copie manuelle
- **🗑️ Effacer** : Vide les résultats et statistiques

## 🔧 Utilisation

### Exemples de base

```bash
# Générer un mot de passe par défaut (16 caractères)
cargo run

# Générer un mot de passe de 20 caractères
cargo run -- --length 20

# Générer un mot de passe sans symboles
cargo run -- --no-symbols

# Générer un mot de passe uniquement avec des lettres et chiffres
cargo run -- --no-symbols --length 12

# Générer 5 mots de passe avec statistiques
cargo run -- --count 5 --stats

# Exclure des caractères spécifiques
cargo run -- --exclude "0O1l" --length 16
```

### Options disponibles

| Option | Description | Défaut |
|--------|-------------|---------|
| `-l, --length <LENGTH>` | Longueur du mot de passe | 16 |
| `--lowercase` | Inclure les lettres minuscules | true |
| `--no-lowercase` | Exclure les lettres minuscules | false |
| `--uppercase` | Inclure les lettres majuscules | true |
| `--no-uppercase` | Exclure les lettres majuscules | false |
| `--numbers` | Inclure les chiffres | true |
| `--no-numbers` | Exclure les chiffres | false |
| `--symbols` | Inclure les caractères spéciaux | true |
| `--no-symbols` | Exclure les caractères spéciaux | false |
| `-e, --exclude <CHARS>` | Caractères spécifiques à exclure | aucun |
| `-c, --count <COUNT>` | Nombre de mots de passe à générer | 1 |
| `-s, --stats` | Afficher les statistiques | false |
| `-h, --help` | Afficher l'aide | - |
| `-V, --version` | Afficher la version | - |

### Exemples avancés

```bash
# Mot de passe pour base de données (pas de caractères problématiques)
cargo run -- --exclude "'\";\\`" --length 24 --stats

# Mot de passe PIN numérique
cargo run -- --no-lowercase --no-uppercase --no-symbols --length 6

# Mot de passe très sécurisé pour admin
cargo run -- --length 32 --stats

# Génération en lot pour équipe
cargo run -- --count 10 --length 16 --stats
```

## 🔒 Sécurité

- Utilise le générateur de nombres aléatoires cryptographiquement sécurisé de Rust
- Garantit la présence d'au moins un caractère de chaque type demandé
- Mélange aléatoire avec l'algorithme Fisher-Yates

## 🛠️ Développement

### Dépendances

- `rand` : Génération de nombres aléatoires sécurisés
- `clap` : Parsing d'arguments en ligne de commande

### Structure du code

- Configuration flexible avec validation
- Séparation claire des responsabilités
- Gestion d'erreurs robuste
- Code documenté et maintenable

### Options disponibles

```
-l, --length <LENGTH>                Longueur du mot de passe [default: 24]
    --include-numbers                Inclure des chiffres
    --exclude-numbers                Exclure des chiffres
    --include-uppercase              Inclure des majuscules
    --exclude-uppercase              Exclure des majuscules
    --include-symbols                Inclure des symboles
    --exclude-symbols                Exclure des symboles
-e, --exclude-chars <EXCLUDE_CHARS>  Caractères spécifiques à exclure
-h, --help                           Afficher l'aide
-V, --version                        Afficher la version
```

## Caractères utilisés

- **Minuscules** : `abcdefghijklmnopqrstuvwxyz`
- **Majuscules** : `ABCDEFGHIJKLMNOPQRSTUVWXYZ`
- **Chiffres** : `0123456789`
- **Symboles** : `!@#$%^&*()-_=+[]{}|;:,.<>?/`

## Sécurité

- Le générateur assure qu'au moins un caractère de chaque type sélectionné est inclus dans le mot de passe
- Utilise l'algorithme Fisher-Yates pour mélanger les caractères
- Utilise `rand::thread_rng()` pour une génération cryptographiquement sûre

## Dépendances

- `rand = "0.8.5"` : Pour la génération de nombres aléatoires
- `clap = { version = "4.0", features = ["derive"] }` : Pour le parsing des arguments

## Structure du projet

```
password_generator_v1.5/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

## Licence

Ce projet est un exercice éducatif dans le cadre d'un cours sur Rust.
