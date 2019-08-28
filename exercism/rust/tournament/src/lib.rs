use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, Clone, Debug)]
struct TeamResults(String, (u32, u32, u32)); // (name, (won, draw, loss))

impl TeamResults {
    fn new(team: String) -> Self {
        Self(team, (0, 0, 0))
    }

    pub fn add_result(&mut self, team_a_result: &str, as_team_a: bool) {
        match team_a_result {
            "win" => {
                if as_team_a {
                    (self.1).0 += 1;
                } else {
                    (self.1).2 += 1;
                }
            }
            "draw" => (self.1).1 += 1,
            "loss" => {
                if as_team_a {
                    (self.1).2 += 1;
                } else {
                    (self.1).0 += 1;
                }
            }
            _ => panic!("Invalid result '{}'", team_a_result),
        }
    }

    pub fn points(&self) -> u32 {
        (self.1).0 * 3 + (self.1).1
    }
}

impl std::cmp::Ord for TeamResults {
    fn cmp(&self, other: &Self) -> Ordering {
        // Numeric on results then alphabetic on name
        self.1.cmp(&other.1).then(other.0.cmp(&self.0))
    }
}

impl std::cmp::PartialOrd for TeamResults {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for TeamResults {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

pub fn tally(match_results: &str) -> String {
    // Compile results
    let mut map: HashMap<String, TeamResults> = HashMap::new();
    for result in match_results.split('\n') {
        let mut i = result.split(';');
        if let (Some(team_a), Some(team_b), Some(a_outcome)) = (i.next(), i.next(), i.next()) {
            map.entry(team_a.to_string())
                .or_insert(TeamResults::new(team_a.to_string()))
                .add_result(a_outcome, true);
            map.entry(team_b.to_string())
                .or_insert(TeamResults::new(team_b.to_string()))
                .add_result(a_outcome, false);
        }
    }
    let mut teams: Vec<TeamResults> = map.drain().map(|(_, v)| v).collect();
    teams.sort();

    // Construct results string
    let mut tally_string = format!(
        "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
        "Team", "MP", "W", "D", "L", "P"
    );
    for team in teams.drain(0..).rev() {
        let (w, d, l) = team.1;
        let (mp, p) = (w + d + l, team.points());
        tally_string.push_str(
            format!(
                "\n{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
                team.0, mp, w, d, l, p
            )
            .as_str(),
        );
    }
    tally_string
}
