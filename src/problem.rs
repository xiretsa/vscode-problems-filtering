use serde::{Deserialize, Serialize};
use tabled::Tabled;

/// Structure représentant un problème VS Code
#[derive(Debug, Deserialize)]
pub struct Problem {

    pub resource: String,

    #[serde(rename = "startLineNumber")]
    pub start_line_number: u32,

    pub message: String,

    // Autres champs optionnels que nous ignorons pour le filtrage
    #[serde(flatten)]
    pub _other: serde_json::Value,
}

/// Structure pour l'affichage en tableau
#[derive(Tabled, Serialize)]
pub struct ProblemOutput {

    #[tabled(rename = "Resource")]
    pub resource: String,

    #[tabled(rename = "Message")]
    pub message: String,

    #[tabled(rename = "Line")]
    pub line: u32,
}

impl ProblemOutput {
    pub fn new(problem: &Problem) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_output_short_path() {
        let problem = Problem {
            resource: "file.txt".to_string(),
            start_line_number: 1,
            message: "test message".to_string(),
            _other: serde_json::Value::Null,
        };

        let output = ProblemOutput::new(&problem);
        assert_eq!(output.resource, "file.txt");
        assert_eq!(output.message, "test message");
        assert_eq!(output.line, 1);
    }

    #[test]
    fn test_problem_output_with_parent() {
        let problem = Problem {
            resource: "src/nested/file.txt".to_string(),
            start_line_number: 1,
            message: "test message".to_string(),
            _other: serde_json::Value::Null,
        };

        let output = ProblemOutput::new(&problem);
        assert_eq!(output.resource, "nested/file.txt");
    }

    #[test]
    fn test_problem_output_long_path() {
        let problem = Problem {
            resource: "/very/long/path/with/many/segments/file.txt".to_string(),
            start_line_number: 1,
            message: "test message".to_string(),
            _other: serde_json::Value::Null,
        };

        let output = ProblemOutput::new(&problem);
        assert_eq!(output.resource, "segments/file.txt");
    }

    #[test]
    fn test_problem_output_short_message() {
        let problem = Problem {
            resource: "file.txt".to_string(),
            start_line_number: 1,
            message: "short message".to_string(),
            _other: serde_json::Value::Null,
        };

        let output = ProblemOutput::new(&problem);
        assert_eq!(output.message, "short message");
    }

    #[test]
    fn test_problem_output_long_message() {
        let message = "a".repeat(200); // Message plus long que 150 caractères
        let problem = Problem {
            resource: "file.txt".to_string(),
            start_line_number: 1,
            message,
            _other: serde_json::Value::Null,
        };

        let output = ProblemOutput::new(&problem);
        assert_eq!(output.message.len(), 150);
        assert!(output.message.ends_with("..."));
    }

    #[test]
    fn test_problem_output_exact_length_message() {
        let message = "a".repeat(150); // Message exactement 150 caractères
        let problem = Problem {
            resource: "file.txt".to_string(),
            start_line_number: 1,
            message: message.clone(),
            _other: serde_json::Value::Null,
        };

        let output = ProblemOutput::new(&problem);
        assert_eq!(output.message, message);
        assert!(!output.message.ends_with("..."));
    }
}
