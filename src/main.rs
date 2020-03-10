use rudim;

fn main() {
    simple_logger::init().unwrap();

    let program = std::fs::read_to_string("test.rdm").unwrap();
    rudim::execute(program.as_str());
}
