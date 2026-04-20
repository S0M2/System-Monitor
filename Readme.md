# MACEON

Ce projet a été réaliser en colabaration avec Claude code ± 65% du code

[](https://www.rust-lang.org/)
[](https://opensource.org/licenses/MIT)
[](https://ratatui.rs/)

Une interface terminal (TUI) haute performance pour le monitoring système et l'optimisation des ressources, développée en Rust. Ce dashboard offre une visibilité complète sur la santé de votre machine avec une esthétique "Cyber" moderne.

## Fonctionnalités

### Monitoring Multi-Onglets

Basculez entre quatre vues spécialisées via la touche `TAB` :

  - **Overview** : Tableau de bord global avec jauges CPU/RAM/Swap et historique graphique (Sparklines).
  - **Processus** : Gestionnaire de tâches complet avec tri dynamique (CPU, Mémoire, PID, Nom).
  - **Connexions** : Analyse des connexions réseau actives avec PID et noms de processus.
  - **Stockage** : **Analyseur hiérarchique ultra-rapide** de fichiers/dossiers avec navigation par répertoire, code couleur par taille, ouverture directe dans Finder.

### Cyber-Sentinel & Alertes

  - **Système d'alertes visuel** : Le bandeau de titre clignote en rouge en cas de surcharge CPU (>85%), RAM critique ou surchauffe thermique.
  - **Thermal Tracking** : Surveillance en temps réel des capteurs de température du SoC (optimisé pour Apple Silicon et x86).
  - **RAM Optimizer** : Algorithme de conseil intégré qui identifie les 5 processus les plus gourmands pour vous aider à libérer de la mémoire.

### Gestion des Processus

  - **Tri intelligent** : Organisez vos processus par consommation de ressources.
  - **Kill switch** : Terminez n'importe quel processus directement depuis l'interface avec la touche `K`.

### Analyseur de Disque (OPTIMISÉ)

#### Caractéristiques
- ✅ **Chargement instantané** : Affiche uniquement le contenu du répertoire courant (zéro récursion)
- ✅ **Fichiers et dossiers** : Vue complète avec code couleur (CYAN = dossiers, LIME = fichiers)
- ✅ **Code couleur par taille** : RED (>10GB), ORANGE (>1GB), LIME (>1MB), GRAY (<1MB)
- ✅ **Navigation rapide** : Flèches ↑↓ pour naviguer, [Enter] pour ouvrir un dossier, [BS] pour revenir
- ✅ **Intégration Finder** : [O] pour révéler le fichier/dossier sélectionné dans Finder
- ✅ **Barres de progression** : Visualisation graphique de la taille relative

#### Hotkeys
- `↑↓` - Naviguer dans la liste
- `Enter` - Ouvrir un dossier
- `Backspace` - Revenir au dossier parent
- `O` - Ouvrir dans Finder
- `Tab` - Changer d'onglet
- `Q` - Quitter

## Installation & Build

### Prérequis

  - [Rust & Cargo](https://rustup.rs/) (dernière version stable)

### Compilation

```bash
# Cloner le projet
git clone https://github.com/yourusername/maceon.git
cd maceon

# Build en mode release pour des performances optimales
cargo build --release

# Lancer l'outil
./target/release/maceon
```

## Raccourcis Clavier

### Navigation Globale
| Touche | Action |
| :--- | :--- |
| `TAB` / `BackTab` | Naviguer entre les onglets |
| `Q` | Quitter l'application |

### Onglet Processus
| Touche | Action |
| :--- | :--- |
| `↑` / `↓` | Naviguer dans la liste |
| `K` | Tuer le processus sélectionné |
| `C` | Trier par **CPU** |
| `M` | Trier par **Mémoire** |
| `P` | Trier par **PID** |
| `N` | Trier par **Nom** |

### Onglet Stockage
| Touche | Action |
| :--- | :--- |
| `↑` / `↓` | Naviguer dans les fichiers/dossiers |
| `Enter` | Ouvrir un dossier |
| `Backspace` | Revenir au dossier parent |
| `O` | Révéler dans Finder |

### Onglet Connections
| Touche | Action |
| :--- | :--- |
| `↑` / `↓` | Scroller la liste |
| `Page Up` / `Page Down` | Scroller rapide |
| `S` | Scanner les connexions |

## Sécurité et Performance

  - **Mémoire Sûre** : Développé 100% en Rust pour garantir l'absence de *buffer overflows* et de *segmentation faults*.
  - **Léger** : Utilise `sysinfo` pour un accès bas niveau aux métriques système avec un impact minimal sur les performances.
  - **Pas de Root** : Fonctionne sans privilèges administrateur (sauf pour tuer des processus système protégés).
  - **Ultra-rapide** : Analyseur de disque instantané (zéro récursion, chargement du répertoire courant uniquement).

## Licence

Distribué sous la licence MIT. Voir le fichier `LICENSE` pour plus d'informations.

*Maceon - System Monitor & Storage Analyzer*
