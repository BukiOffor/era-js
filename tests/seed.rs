#![allow(unused_variables)]
fn main() {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }

    #[wasm_bindgen_test]
    fn fail() {
        assert_eq!(1, 1);
    }
}
