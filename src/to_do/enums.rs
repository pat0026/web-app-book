use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::DONE => "DONE".to_string(),
            Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn new(input_str: &str) -> Self {
        match input_str {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_str),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        Self::new(input_string.as_str())
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DONE => write!(f, "DONE"),
            Self::PENDING => write!(f, "PENDING"),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}


// fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// where S: Serializer,
// {
//     let mut s = serializer.serialize_struct("TaskStatus", 1)?;
//     s.serialize_field("status", &self.stringify())?;
//     s.end()
// }

// #[derive(Serialize)]
// struct TaskStatus {
//     status: String
// }