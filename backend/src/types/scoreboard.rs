use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScoreboardTeam {
    pub name: String,
    pub score: i32,
}
