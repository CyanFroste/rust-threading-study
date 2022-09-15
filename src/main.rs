mod file_operations;

fn main() {
    file_operations::read_write_to_same_file().expect(":)");
}
