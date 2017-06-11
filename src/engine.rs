pub enum MatchResult {
    Yes,
    No,
}

pub fn m(a: &str, b: &str) -> bool {
    if !a.starts_with("^"){
        return b.contains(a)
    } else {
        return b.starts_with(&a[1..])
    }
}