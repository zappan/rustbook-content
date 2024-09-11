use adder;

mod common;

// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
// Cargo treats the tests/ directory specially and compiles files in this directory
// only when we run cargo test. Run cargo test now:

#[test]
fn integration_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}
