use rand::Rng;
use std::collections::HashSet;

slint::include_modules!();

#[derive(Debug, Clone)]
struct PasswordConfig {
    length: usize,
    include_lowercase: bool,
    include_uppercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    excluded_chars: HashSet<char>,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Callback pour générer les mots de passe
    ui.on_generate_password({
        let ui_weak = ui.as_weak();
        move || {
            let ui = ui_weak.unwrap();

            // Récupération des paramètres depuis l'interface
            let config = PasswordConfig {
                length: ui.get_password_length() as usize,
                include_lowercase: ui.get_include_lowercase(),
                include_uppercase: ui.get_include_uppercase(),
                include_numbers: ui.get_include_numbers(),
                include_symbols: ui.get_include_symbols(),
                excluded_chars: ui.get_excluded_chars().chars().collect(),
            };

            // Validation des paramètres
            if config.length == 0 {
                ui.set_generated_passwords(
                    "❌ Erreur: La longueur doit être supérieure à 0".into(),
                );
                return;
            }

            if !config.include_lowercase
                && !config.include_uppercase
                && !config.include_numbers
                && !config.include_symbols
            {
                ui.set_generated_passwords(
                    "❌ Erreur: Au moins un type de caractère doit être inclus".into(),
                );
                return;
            }

            let count = ui.get_password_count() as usize;

            let mut results = Vec::new();

            // Génération des mots de passe
            for i in 1..=count {
                let password = generate_password(&config);

                if password.starts_with("❌") {
                    ui.set_generated_passwords(password.into());
                    return;
                }

                if count > 1 {
                    results.push(format!("Mot de passe #{}: {}", i, password));
                } else {
                    results.push(format!("Mot de passe: {}", password));
                }
            }

            // Affichage des résultats
            ui.set_generated_passwords(results.join("\n").into());
        }
    });

    // Callback pour copier dans le presse-papiers
    let ui_weak_copy = ui.as_weak();
    ui.on_copy_to_clipboard(move |_text| {
        let ui = ui_weak_copy.unwrap();

        // Pour une implémentation complète du presse-papiers,
        // vous pourriez utiliser une crate comme `clipboard`

        // Pour l'instant, on affiche juste un message
        let mut current_text = ui.get_generated_passwords().to_string();
        current_text.push_str("\n\n📋 Texte prêt à être copié manuellement");
        ui.set_generated_passwords(current_text.into());
    });

    ui.run()
}

fn generate_password(config: &PasswordConfig) -> String {
    let mut rng = rand::thread_rng();

    // Définition des ensembles de caractères
    let lowercase_chars = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let number_chars = "0123456789";
    let symbol_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?/~`";

    // Construction de l'ensemble de caractères disponibles
    let mut available_chars = String::new();
    let mut required_chars = Vec::new();

    if config.include_lowercase {
        available_chars.push_str(lowercase_chars);
        if let Some(ch) =
            get_random_char_from_set(lowercase_chars, &config.excluded_chars, &mut rng)
        {
            required_chars.push(ch);
        }
    }

    if config.include_uppercase {
        available_chars.push_str(uppercase_chars);
        if let Some(ch) =
            get_random_char_from_set(uppercase_chars, &config.excluded_chars, &mut rng)
        {
            required_chars.push(ch);
        }
    }

    if config.include_numbers {
        available_chars.push_str(number_chars);
        if let Some(ch) = get_random_char_from_set(number_chars, &config.excluded_chars, &mut rng) {
            required_chars.push(ch);
        }
    }

    if config.include_symbols {
        available_chars.push_str(symbol_chars);
        if let Some(ch) = get_random_char_from_set(symbol_chars, &config.excluded_chars, &mut rng) {
            required_chars.push(ch);
        }
    }

    // Filtrer les caractères exclus
    let filtered_chars: Vec<char> = available_chars
        .chars()
        .filter(|c| !config.excluded_chars.contains(c))
        .collect();

    if filtered_chars.is_empty() {
        return "❌ Erreur: Aucun caractère disponible pour générer le mot de passe".to_string();
    }

    // Vérifier que nous avons assez d'espace pour les caractères requis
    if required_chars.len() > config.length {
        return format!(
            "❌ Erreur: La longueur ({}) est trop courte pour inclure tous les types de caractères requis ({})",
            config.length,
            required_chars.len()
        );
    }

    let mut password_chars = required_chars;

    // Compléter avec des caractères aléatoires
    while password_chars.len() < config.length {
        let random_char = filtered_chars[rng.gen_range(0..filtered_chars.len())];
        password_chars.push(random_char);
    }

    // Mélanger le mot de passe avec l'algorithme Fisher-Yates
    for i in (1..password_chars.len()).rev() {
        let j = rng.gen_range(0..=i);
        password_chars.swap(i, j);
    }

    password_chars.into_iter().collect()
}

fn get_random_char_from_set(
    charset: &str,
    excluded: &HashSet<char>,
    rng: &mut impl Rng,
) -> Option<char> {
    let available: Vec<char> = charset.chars().filter(|c| !excluded.contains(c)).collect();

    if available.is_empty() {
        None
    } else {
        Some(available[rng.gen_range(0..available.len())])
    }
}


