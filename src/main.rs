fn main() {
    let pdf = parser::parser::parse(include_bytes!("../test.pdf")).unwrap();
    slint_tree::main(pdf);
}
