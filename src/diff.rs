use serde::{Serialize, Serializer};
use unidiff::{Hunk, Line, PatchedFile};

// Define a new type
pub struct SerializableLine(Line);

// Implement Serialize for the new type
impl Serialize for SerializableLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the inner Line
        let line_as_string = format!("{}", self.0);
        serializer.serialize_str(&line_as_string)
    }
}

pub struct SerializableHunk(Hunk);

impl Serialize for SerializableHunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the inner Line
        let line_as_string = format!("{}", self.0);
        serializer.serialize_str(&line_as_string)
    }
}

#[derive(Debug)]
pub struct SerializablePatchedFile(pub PatchedFile);

impl Serialize for SerializablePatchedFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the inner Line
        let line_as_string = format!("{}", self.0);
        serializer.serialize_str(&line_as_string)
    }
}
