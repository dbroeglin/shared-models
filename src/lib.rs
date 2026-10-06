use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub completed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateTask {
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthResponse {
    pub service: String,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl CreateTask {
    pub fn normalized_title(&self) -> Option<String> {
        let title = self.title.trim();
        (!title.is_empty()).then(|| title.to_owned())
    }

    pub fn normalized_content(&self) -> Option<String> {
        let content = self.content.trim();
        (!content.is_empty()).then(|| content.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_non_empty_titles() {
        let request = CreateTask {
            title: "  Ship the demo  ".into(),
            content: "  Coordinate all repositories  ".into(),
        };

        assert_eq!(request.normalized_title().as_deref(), Some("Ship the demo"));
        assert_eq!(
            request.normalized_content().as_deref(),
            Some("Coordinate all repositories")
        );
    }

    #[test]
    fn rejects_blank_fields() {
        let request = CreateTask {
            title: "   ".into(),
            content: "   ".into(),
        };

        assert_eq!(request.normalized_title(), None);
        assert_eq!(request.normalized_content(), None);
    }
}
