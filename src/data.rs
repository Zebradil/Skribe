
use uuid::Uuid;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl Default for DateTime {
    fn default() -> Self {
        Self(chrono::Utc::now())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id(Uuid);

impl Deref for Id {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Item {
    pub id: Id,
    pub text: String,
    pub title: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deadline_at: Option<DateTime>,
    pub wait_until: Option<DateTime>,
    pub priority: f64,
    pub is_deleted: bool,
}

impl Item {}