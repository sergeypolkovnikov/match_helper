use super::probability::Probability;

/// Вектор вероятностей. Сумма всех элементов равна 1.
/// 
/// # Examples
/// 
/// ```
/// let mut v = ProbVec::new(2);
/// v.add(Probability::from(1.0));
/// assert_eq!(v[1], Probability::zero());
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct ProbVec(Vec<Probability>);

impl ProbVec {
    pub fn new(capacity: usize) -> ProbVec {
        let mut result = ProbVec(Vec::with_capacity(capacity));
        result.0.push(Probability::from(1.0));
        result
    }

    fn push(& mut self) {
        self.0.push(Probability::from(0.0))
    }

    pub fn add(&mut self, prob: Probability) {
        self.push();
        for i in (0 .. self.0.len() - 1).rev() {
            self.0[i + 1] = (self.0[i + 1] * prob.rev()) + (self.0[i] * prob);
        }
        self.0[0] = self.0[0] * prob;
    }

    pub fn get_match_prob(&self) -> MatchProb {
        let draw_zone = self.0.len() / 2;
        let prob_to_draw = if self.0.len() % 2 == 0 { Probability::zero() }
            else { self.0[draw_zone] };
        let prob_to_lose = self.0.iter().take(draw_zone).sum();
        MatchProb{ prob_to_win: prob_to_lose, prob_to_draw: prob_to_draw }.rev()
    }
}

pub struct MatchProb
{
    prob_to_win: Probability,
    prob_to_draw: Probability
}

impl MatchProb
{
    pub fn rev(self) -> MatchProb {
        MatchProb{ prob_to_win: (self.prob_to_win + self.prob_to_draw).rev()
            , prob_to_draw: self.prob_to_draw}
    }

    pub fn prob_to_lose(&self) -> Probability {
        (self.prob_to_win + self.prob_to_draw).rev()
    }

    pub fn prob_to_win(&self) -> Probability {
        self.prob_to_win
    }
    
    pub fn prob_to_draw(&self) -> Probability {
        self.prob_to_draw
    }
}
