#[allow(dead_code)]
fn main() {
    ["first", "apple", "   ", "  d"]
        .iter()
        .map(|word| transform(word))
        .for_each(|transformed_word| println!("{}", transformed_word));
}

fn transform(word: &str) -> String {
    match word {
        _ if word.trim().len() < 2 => word.to_string(),
        word => {
            let (first, _) = word.split_at(1);
            match "aeiou".contains(&first) {
                true => format!("{}-hay", word),
                false => format!("{}-{}ay", word.chars().skip(1).collect::<String>(), first),
            }
        }
    }
}
