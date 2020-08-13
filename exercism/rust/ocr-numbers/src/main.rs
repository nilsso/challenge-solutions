use ocr_numbers::convert;

fn main() {
    let input = " _  _ \n".to_string()
        + "| || |\n"
        + "|_||_|\n"
        + "      \n"
        + " _  _ \n"
        + "| || |\n"
        + "|_||_|\n"
        + "      ";

    println!("{}", convert(&input).unwrap());
}
