fn main() {
    let tree = parser::parser::parse(include_bytes!("../test.pdf")).unwrap();
    slint_tree::main(tree);
}
