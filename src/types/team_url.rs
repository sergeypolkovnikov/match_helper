use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Ord, Deserialize)]
pub struct TeamUrl(String);

impl TeamUrl {
    pub fn to_team_name(&self) -> &str {
        let start = "https://api.chess.com/pub/club/";
        &self.0[start.len()..]
    }
}