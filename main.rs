fn main() {
    let secret_word = "thinkpad";
    let mut guess = String::new();

    println!("===  RUST FUN GAME  ===");
    println!("Tebak kata rahasia (hint: laptop favorit Angelice):");

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Gagal membaca input");

    if guess.trim().to_lowercase() == secret_word {
        println!("LUAR BIASA! Kamu menebaknya dengan benar!");
    } else {
        println!("Salah! Kata rahasianya adalah: {}", secret_word);
    }
}
