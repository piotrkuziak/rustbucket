use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
    Unknown
}
#[derive(Debug, Serialize)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub status: Status
}

impl Task {
    pub fn new(id: usize, name: String) -> Self {
        Self { id: id, name, status: Status::ToDo }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::ToDo => write!(f, "To do"),
            Status::InProgress => write!(f, "In progress"),
            Status::Done => write!(f, "Done"),
            Status::Unknown => write!(f, "Wrong status value")
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {}. {}: {}", self.id, self.name, self.status)
    }
}
