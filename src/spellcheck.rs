use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use actix_web::{get, web::Path, HttpResponse};

pub fn read_dict() -> Vec<String> {
    let file = File::open("rsc/words.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[get("/check/{text}")]
pub async fn check(path: Path<(String,)>) -> HttpResponse {
    let word = path.into_inner().0;
    let better_words = check_against_all(&word);

    HttpResponse::Ok().json(better_words)
}

fn check_against_all(word: &str) -> Vec<String> {
    let mut correct_words = read_dict();

    correct_words.sort_by(|w, o| diff(word, w).cmp(&diff(word, o)));

    return correct_words.clone().iter().take(5).cloned().collect();
}

fn diff(word1: &str, word2: &str) -> usize {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize the first row and column of the matrix
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    // Fill in the rest of the matrix
    for (i, char1) in word1.chars().enumerate() {
        for (j, char2) in word2.chars().enumerate() {
            let cost = if char1 == char2 { 0 } else { 1 };
            dp[i + 1][j + 1] = (dp[i][j] + cost) // match or substitution
                .min(dp[i][j + 1] + 1) // insertion
                .min(dp[i + 1][j] + 1); // deletion
        }
    }

    // Return the final cell value, which represents the minimum edit distance
    dp[len1][len2]
}
