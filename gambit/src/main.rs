fn main() {
    let handle = gambit_engine::spawn();
    gambit_uci::run(handle);
}
