use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateTask {
    pub title: String,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_non_empty_titles() {
        let request = CreateTask {
            title: "  Ship the demo  ".into(),
        };

        assert_eq!(request.normalized_title().as_deref(), Some("Ship the demo"));
    }

    #[test]
    fn rejects_blank_titles() {
        let request = CreateTask {
            title: "   ".into(),
        };

        assert_eq!(request.normalized_title(), None);
    }
}
