extern crate rpcre;

#[test]
fn simple() {
    assert!(rpcre::m("d1", "d12"));
}