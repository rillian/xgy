// Text conversion for Ancient Egyptian.

fn main() {
    for arg in std::env::args().skip(1) {
        let mdc = arg.replace("ꜣ", "A")
                     .replace("ꜥ", "a")
                     .replace("š", "S")
                     .replace("ḥ", "H")
                     .replace("ḫ", "x")
                     .replace("ẖ", "X")
                     .replace("ṯ", "T")
                     .replace("ḏ", "D");
        println!("arg: {} -> {}", arg, mdc);
    }
    println!("That's all!");
}
