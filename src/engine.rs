pub enum MatchResult {
    Yes,
    No,
}

pub fn m(a: &str, b: &str) -> bool {
    b.contains(a)
}