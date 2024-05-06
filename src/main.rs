#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut scores = self.scores.to_owned();
        scores.sort();
        scores.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_owned();
        scores.sort();

        return scores.iter().rev().take(3).cloned().collect();
    }
}

fn main() {
    let expected = &[40, 20, 40, 30];
    let crap = HighScores::new(expected);
    println!("{:?}", crap.personal_top_three());
}
