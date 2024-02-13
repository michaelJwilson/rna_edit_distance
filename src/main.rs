use rna_edit_distance::levenshtein_distance;

fn main() {
    let s1: &str = "kitten";
    let s2: &str = "sitting";

    let distance: Result<u64, &str> = levenshtein_distance(s1, s2);

    if let Ok(value) = distance {
        println!(
            "The Levenshtein distance between '{}' and '{}' is {}",
            s1, s2, value
        );
    } else if let Err(err) = distance {
        println!("Oops, there is an error: {}", err);
    }
}
