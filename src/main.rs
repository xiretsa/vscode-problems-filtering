mod problem;

use anyhow::{Context, Result};
use clap::Parser;
use problem::{Problem, ProblemOutput};
use std::fs;
use std::path::PathBuf;
use tabled::{Table};
use std::io::Write;

/// Application CLI pour filtrer les problèmes VS Code
#[derive(Parser)]
#[command(
    name = "vscode-problems-filtering",
    about = "Filtre les problèmes VS Code selon des critères d'inclusion et d'exclusion",
    version = "0.1.0"
)]
struct CliProblemApp {
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

impl CliProblemApp {
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
    let cli = CliProblemApp::parse();

    // Utiliser stdout comme writer pour l'exécution normale
    let mut stdout = std::io::stdout();
    run_app(
        &cli,
    |p: &PathBuf| fs::read_to_string(p).with_context(|| format!("Impossible de lire le fichier: {p:?}")),
        &mut stdout,
    )
}

/// Function extracted from `main` to allow injecting a reader and an output writer for tests.
fn run_app<F, W>(
    cli: &CliProblemApp,
    read_fn: F,
    out: &mut W,
) -> Result<()>
where
    F: Fn(&PathBuf) -> Result<String>,
    W: Write,
{
    // Validation des arguments
    if cli.include_terms.is_empty() && cli.exclude_terms.is_empty() {
        anyhow::bail!("Au moins un terme d'inclusion ou d'exclusion doit être spécifié");
    }

    // Lecture et parsing du fichier JSON
    let file_content = read_fn(&cli.input)?;

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
        writeln!(out, "{json_output}")?;
        return Ok(());
    }

    writeln!(out, "Nombre total de problèmes: {}", problems.len())?;

    if !cli.include_terms.is_empty() {
        writeln!(out, "Termes à inclure: {}", cli.include_terms.join(", "))?;
    }

    if !cli.exclude_terms.is_empty() {
        writeln!(out, "Termes à exclure: {}", cli.exclude_terms.join(", "))?;
    }

    if cli.ignore_case {
        writeln!(out, "Mode insensible à la casse activé")?;
    }

    writeln!(out)?;

    writeln!(out, "Nombre de problèmes filtrés: {}", filtered_problems.len())?;

    if cli.count_only {
        return Ok(());
    }

    writeln!(out)?;

    // Affichage du tableau
    if filtered_problems.is_empty() {
        writeln!(out, "Aucun problème ne correspond aux critères de filtrage.")?;
    } else {
        let table = Table::new(&filtered_problems);
        writeln!(out, "{table}")?;
    }

    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_app_json_output() {
        let cli = CliProblemApp {
            input: PathBuf::from("does_not_matter.json"),
            include_terms: vec!["deprecated".to_string()],
            exclude_terms: vec![],
            ignore_case: false,
            count_only: false,
            json: true,
        };

        // JSON in-memory with one problem matching
        let json = r#"[
            { "resource": "a/test.java", "startLineNumber": 1, "message": "This is deprecated" }
        ]"#;

        let read_fn = |_p: &PathBuf| Ok(json.to_string());

        let mut out = Vec::new();
        run_app(&cli, read_fn, &mut out).expect("run_app failed");
        let s = String::from_utf8(out).expect("invalid utf8");
        assert!(s.contains("This is deprecated"));
        assert!(s.trim_start().starts_with('['));
    }

    #[test]
    fn test_run_app_count_only() {
        let cli = CliProblemApp {
            input: PathBuf::from("x.json"),
            include_terms: vec!["foo".to_string()],
            exclude_terms: vec![],
            ignore_case: false,
            count_only: true,
            json: false,
        };

        let json = r#"[
            { "resource": "a/test.java", "startLineNumber": 1, "message": "foo bar" },
            { "resource": "b/test.java", "startLineNumber": 2, "message": "no match" }
        ]"#;

        let read_fn = |_p: &PathBuf| Ok(json.to_string());

        let mut out = Vec::new();
        run_app(&cli, read_fn, &mut out).expect("run_app failed");
        let s = String::from_utf8(out).expect("invalid utf8");
        assert!(s.contains("Nombre total de problèmes: 2"));
        assert!(s.contains("Nombre de problèmes filtrés: 1"));
    }

    #[test]
    fn test_run_app_validation_error() {
        let cli = CliProblemApp {
            input: PathBuf::from("x.json"),
            include_terms: vec![],
            exclude_terms: vec![],
            ignore_case: false,
            count_only: false,
            json: false,
        };

        let read_fn = |_p: &PathBuf| Ok("[]".to_string());

        let mut out = Vec::new();
        let res = run_app(&cli, read_fn, &mut out);
        assert!(res.is_err());
        // Vérifier que le message d'erreur correspond à la validation des arguments
        if let Err(e) = res {
            let msg = format!("{e}");
            assert!(msg.contains("Au moins un terme d'inclusion ou d'exclusion doit être spécifié"), "unexpected error message: {msg}");
        }
    }

    #[test]
    fn test_run_app_table_output() {
        let cli = CliProblemApp {
            input: PathBuf::from("x.json"),
            include_terms: vec!["matchme".to_string()],
            exclude_terms: vec![],
            ignore_case: false,
            count_only: false,
            json: false,
        };

        let json = r#"[
            { "resource": "a/test.java", "startLineNumber": 3, "message": "matchme here" }
        ]"#;

        let read_fn = |_p: &PathBuf| Ok(json.to_string());

        let mut out = Vec::new();
        run_app(&cli, read_fn, &mut out).expect("run_app failed");
        let s = String::from_utf8(out).expect("invalid utf8");
        // Table should contain header Resource and Message (tabled derives these names)
        assert!(s.contains("Resource"));
        assert!(s.contains("Message"));
        assert!(s.contains("matchme here"));
    }

    #[test]
    fn test_run_app_ignore_case_include() {
        let cli = CliProblemApp {
            input: PathBuf::from("x.json"),
            include_terms: vec!["DEPRECATED".to_string()],
            exclude_terms: vec![],
            ignore_case: true,
            count_only: false,
            json: false,
        };

        let json = r#"[
            { "resource": "a/test.java", "startLineNumber": 4, "message": "this is deprecated" }
        ]"#;

        let read_fn = |_p: &PathBuf| Ok(json.to_string());

        let mut out = Vec::new();
        run_app(&cli, read_fn, &mut out).expect("run_app failed");
        let s = String::from_utf8(out).expect("invalid utf8");
        assert!(s.contains("Nombre de problèmes filtrés: 1"));
    }

    #[test]
    fn test_run_app_ignore_case_exclude() {
        let cli = CliProblemApp {
            input: PathBuf::from("x.json"),
            include_terms: vec![],
            exclude_terms: vec!["WARNING".to_string()],
            ignore_case: true,
            count_only: false,
            json: false,
        };

        let json = r#"[
            { "resource": "a/test.java", "startLineNumber": 4, "message": "this is a warning" }
        ]"#;

        let read_fn = |_p: &PathBuf| Ok(json.to_string());

        let mut out = Vec::new();
        run_app(&cli, read_fn, &mut out).expect("run_app failed");
        let s = String::from_utf8(out).expect("invalid utf8");
        // Since the only problem matches the exclude term, filtered count should be 0
        assert!(s.contains("Nombre de problèmes filtrés: 0"));
    }

    #[test]
    fn test_filter_problem_include() {
        let cli = CliProblemApp {
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
        let cli = CliProblemApp {
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
        let cli = CliProblemApp {
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

    #[test]
    fn test_filter_problem_case_sensitive() {
        let cli = CliProblemApp {
            input: PathBuf::new(),
            include_terms: vec!["DEPRECATED".to_string()],
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

        assert!(!cli.filter_problem(&problem));
    }

    #[test]
    fn test_filter_problem_exclude_case_insensitive() {
        let cli = CliProblemApp {
            input: PathBuf::new(),
            include_terms: vec![],
            exclude_terms: vec!["WARNING".to_string()],
            ignore_case: true,
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
    fn test_filter_problem_exclude_case_sensitive() {
        let cli = CliProblemApp {
            input: PathBuf::new(),
            include_terms: vec![],
            exclude_terms: vec!["WARNING".to_string()],
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

        assert!(cli.filter_problem(&problem));
    }

}
