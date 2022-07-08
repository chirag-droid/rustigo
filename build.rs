extern crate lalrpop;

fn main() {
  // generate the lalrpop file in the source tree
  lalrpop::Configuration::new()
    .generate_in_source_tree()
    .process()
    .unwrap();
}
