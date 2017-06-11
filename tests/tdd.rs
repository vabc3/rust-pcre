extern crate rpcre;

#[test]
fn simple() {
    run_match("d1", vec![ "d12", "d18" ], vec![ "d212", "d8" ]);
}

fn run_match(pattern: &'static str, positive_values: Vec<&'static str>, negative_values: Vec<&'static str>) {
    for val in positive_values {
        assert!(rpcre::m(pattern, val), format!("'{}' should match '{}'", pattern, val));
    }

    for val in &negative_values {
        assert!(!rpcre::m(pattern, val), format!("'{}' should not match '{}'", pattern, val));
    }
}