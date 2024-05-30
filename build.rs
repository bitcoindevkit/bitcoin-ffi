fn main() {
    uniffi::generate_scaffolding("./src/bitcoin.udl").unwrap();
}
