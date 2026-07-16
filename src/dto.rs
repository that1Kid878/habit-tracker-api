pub struct CreateHabitRequest {
    pub username: String,
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub days: Vec<i64>,
}

pub struct EditHabitRequest {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub days: Option<Vec<i64>>,
}
