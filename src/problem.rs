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
