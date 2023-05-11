use futures::executor::block_on;
use serde::Deserialize;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

pub mod types;

use crate::types::probability::Probability;
use crate::types::prob_vec::ProbVec;
use crate::types::prob_vec::MatchProb;
use crate::types::rating::Rating;

#[tokio::main]
async fn main() {
    let team = "kharkiv-chess-team";
    match block_on(read_matches(team))
    {
        Ok(matches) => {
            println!("Matches count = {}", matches.len());
            let mut col = matches.iter()
                .map(|m| print_match_prob(team, &m))
                .collect::<FuturesUnordered<_>>();
            while let Some(_) = col.next().await {

            }
        }
        Err(e) => println!("Error: {}", e)
    }
}

trait WinProb<T = Self> {
    fn win_prob(&self, other: &Self) -> Probability;
}

impl WinProb for Rating {
    fn win_prob(&self, other: &Self) -> Probability {
        Probability::from(1_f64 / (1_f64 + 10_f64.powf((*self - *other) as f64 / 400_f64))) 
    }
}

impl WinProb for Player {
    fn win_prob(&self, other: &Self) -> Probability {
        self.rating.win_prob(&other.rating)
    }
}

#[derive(Deserialize)]
struct Player
{
    rating: Rating,
}

#[derive(Deserialize)]
struct Team
{
    players: Vec<Player>,
}

#[derive(Deserialize)]
struct Teams
{
    team1: Team,
    team2: Team,
}

#[derive(Deserialize)]
struct Match
{
    teams: Teams,
}

#[derive(Deserialize)]
struct TeamMatch
{
    #[serde(rename(deserialize = "@id"))]
    id: String,
    #[serde(rename(deserialize = "opponent"))]
    opponent_url: String
}

impl TeamMatch {
    fn opponent(&self) -> &str {
        let start = "https://api.chess.com/pub/club/";
        &self.opponent_url[start.len()..]
    }
}

#[derive(Deserialize)]
struct TeamMatches
{
    registered: Vec<TeamMatch>
}

async fn read_matches(team: &str) -> Result<Vec<TeamMatch>, Box<dyn std::error::Error>>
{
    let req = reqwest::get(format!("https://api.chess.com/pub/club/{}/matches", team)).await?;
    let matches = req.json::<TeamMatches>().await?;
    Ok(matches.registered)
}

async fn print_match_prob(team: &str, team_match: &TeamMatch) -> () {
    if let Ok(prob) = calc_match_prob(&team_match.id).await {
        let prob = if team != team_match.opponent() { prob } else { prob.rev() };
        println!("Match with '{}' : win {}, draw {}, lose {}"
            , team_match.opponent()
            , prob.prob_to_win()
            , prob.prob_to_draw()
            , prob.prob_to_lose());
    } 
    else { println!("Error"); }
}

async fn calc_match_prob(link: &str) -> Result<MatchProb, Box<dyn std::error::Error>> {
    let req = reqwest::get(link).await?;
    let mut match_ = req.json::<Match>().await?;
    match_.teams.team1.players.sort_by_key(|p| p.rating);
    match_.teams.team2.players.sort_by_key(|p| p.rating);
    let probs = match_.teams.team1.players.iter()
        .zip(match_.teams.team2.players.iter())
        .map(|(fst, snd)| fst.win_prob(snd))
        .fold(ProbVec::new(match_.teams.team1.players.len() * 2 + 1), |mut v, p| { v.add(p); v.add(p); v }); // double call: white & black
    Ok(probs.get_match_prob())
}
