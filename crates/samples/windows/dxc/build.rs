use windows_bindgen::bindgen;

fn main() {
    let warnings = bindgen(["--etc", "bindings.txt"]);
    println!("{warnings}");
}
