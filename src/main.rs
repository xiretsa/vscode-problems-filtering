use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tabled::{Table, Tabled};

/// Structure représentant un problème VS Code
#[derive(Debug, Deserialize)]
struct Problem {
    resource: String,
    #[serde(rename = "startLineNumber")]
    start_line_number: u32,
    message: String,
    // Autres champs optionnels que nous ignorons pour le filtrage
    #[serde(flatten)]
    _other: serde_json::Value,
}

/// Structure pour l'affichage en tableau
#[derive(Tabled, Serialize)]
struct ProblemOutput {
    #[tabled(rename = "Resource")]
    resource: String,
    #[tabled(rename = "Message")]
    message: String,
    #[tabled(rename = "Line")]
    line: u32,
}

impl ProblemOutput {
    fn new(problem: &Problem) -> Self {
        // Tronquer le chemin pour l'affichage (garder seulement le nom du fichier et le dossier parent)
        let resource = if let Some(pos) = problem.resource.rfind('/') {
            let filename = &problem.resource[pos + 1..];
            // Essayer de garder aussi le dossier parent
            if let Some(parent_pos) = problem.resource[..pos].rfind('/') {
                let parent = &problem.resource[parent_pos + 1..pos];
                format!("{parent}/{filename}")
            } else {
                filename.to_string()
            }
        } else {
            problem.resource.clone()
        };

        // Tronquer le message s'il est trop long
        let message = if problem.message.len() > 150 {
            format!("{}...", &problem.message[..147])
        } else {
            problem.message.clone()
        };

        Self {
            resource,
            message,
            line: problem.start_line_number,
        }
    }
}

/// Application CLI pour filtrer les problèmes VS Code
#[derive(Parser)]
#[command(
    name = "vscode-problems-filtering",
    about = "Filtre les problèmes VS Code selon des critères d'inclusion et d'exclusion",
    version = "0.1.0"
)]
struct Cli {
    /// Fichier JSON contenant les problèmes VS Code
    #[arg(short = 'f', long, value_name = "FILE")]
    input: PathBuf,

    /// Termes à inclure (tous doivent être présents dans le message)
    #[arg(short = 'i', long = "include", value_name = "TERM")]
    include_terms: Vec<String>,

    /// Termes à exclure (aucun ne doit être présent dans le message)
    #[arg(short = 'e', long = "exclude", value_name = "TERM")]
    exclude_terms: Vec<String>,

    /// Ignorer la casse lors de la comparaison
    #[arg(long)]
    ignore_case: bool,

    /// Afficher seulement le nombre de résultats (pas le tableau)
    #[arg(short = 'c', long)]
    count_only: bool,

    /// Sortie au format JSON
    #[arg(long)]
    json: bool,
}

impl Cli {
    /// Filtre un problème selon les critères d'inclusion et d'exclusion
    fn filter_problem(&self, problem: &Problem) -> bool {
        let message = if self.ignore_case {
            problem.message.to_lowercase()
        } else {
            problem.message.clone()
        };

        // Vérifier que tous les termes d'inclusion sont présents
        let all_include_present = self.include_terms.iter().all(|term| {
            let search_term = if self.ignore_case {
                term.to_lowercase()
            } else {
                term.clone()
            };
            message.contains(&search_term)
        });

        // Vérifier qu'aucun terme d'exclusion n'est présent
        let no_exclude_present = self.exclude_terms.iter().all(|term| {
            let search_term = if self.ignore_case {
                term.to_lowercase()
            } else {
                term.clone()
            };
            !message.contains(&search_term)
        });

        all_include_present && no_exclude_present
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validation des arguments
    if cli.include_terms.is_empty() && cli.exclude_terms.is_empty() {
        anyhow::bail!("Au moins un terme d'inclusion ou d'exclusion doit être spécifié");
    }

    // Lecture et parsing du fichier JSON
    let file_content = fs::read_to_string(&cli.input)
        .with_context(|| format!("Impossible de lire le fichier: {:?}", cli.input))?;

    let problems: Vec<Problem> =
        serde_json::from_str(&file_content).with_context(|| "Erreur lors du parsing du JSON")?;

    // Filtrage des problèmes
    let filtered_problems: Vec<ProblemOutput> = problems
        .iter()
        .filter(|problem| cli.filter_problem(problem))
        .map(ProblemOutput::new)
        .collect();

    if cli.json {
        let json_output = serde_json::to_string_pretty(&filtered_problems)
            .with_context(|| "Erreur lors de la sérialisation JSON")?;
        println!("{json_output}");
        return Ok(());
    }

    println!("Nombre total de problèmes: {}", problems.len());

    if !cli.include_terms.is_empty() {
        println!("Termes à inclure: {}", cli.include_terms.join(", "));
    }

    if !cli.exclude_terms.is_empty() {
        println!("Termes à exclure: {}", cli.exclude_terms.join(", "));
    }

    if cli.ignore_case {
        println!("Mode insensible à la casse activé");
    }

    println!();

    println!("Nombre de problèmes filtrés: {}", filtered_problems.len());

    if cli.count_only {
        return Ok(());
    }

    println!();

    // Affichage du tableau
    if filtered_problems.is_empty() {
        println!("Aucun problème ne correspond aux critères de filtrage.");
    } else {
        let table = Table::new(&filtered_problems);
        println!("{table}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_problem_include() {
        let cli = Cli {
            input: PathBuf::new(),
            include_terms: vec!["deprecated".to_string()],
            exclude_terms: vec![],
            ignore_case: false,
            count_only: false,
            json: false,
        };

        let problem = Problem {
            resource: "test.java".to_string(),
            start_line_number: 10,
            message: "The type ActionError is deprecated".to_string(),
            _other: serde_json::Value::Null,
        };

        assert!(cli.filter_problem(&problem));
    }

    #[test]
    fn test_filter_problem_exclude() {
        let cli = Cli {
            input: PathBuf::new(),
            include_terms: vec![],
            exclude_terms: vec!["warning".to_string()],
            ignore_case: false,
            count_only: false,
            json: false,
        };

        let problem = Problem {
            resource: "test.java".to_string(),
            start_line_number: 10,
            message: "This is a warning message".to_string(),
            _other: serde_json::Value::Null,
        };

        assert!(!cli.filter_problem(&problem));
    }

    #[test]
    fn test_filter_problem_case_insensitive() {
        let cli = Cli {
            input: PathBuf::new(),
            include_terms: vec!["DEPRECATED".to_string()],
            exclude_terms: vec![],
            ignore_case: true,
            count_only: false,
            json: false,
        };

        let problem = Problem {
            resource: "test.java".to_string(),
            start_line_number: 10,
            message: "The type ActionError is deprecated".to_string(),
            _other: serde_json::Value::Null,
        };

        assert!(cli.filter_problem(&problem));
    }
}
