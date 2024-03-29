use std::collections::{HashMap, HashSet};

pub fn get_word_list() -> HashSet<String> {
    let file_contents = include_str!("../resources/dict.txt");

    let word_search: HashSet<String> = file_contents
        .lines()
        .map(|line| line.to_lowercase())
        .collect();

    word_search
}

#[derive(Debug)]
pub struct SolveResult {
    pub solution: String,
    pub words_used: Vec<String>,
}

// Get valid words based on puzzle parts and word set
fn get_valid_words<'a>(
    parts: &'a [&'a str],
    size: usize,
    ws: &'a HashSet<String>,
) -> HashSet<&'a str> {
    let min_length: usize = parts.iter().map(|p| p.len()).min().unwrap_or(0) + size;
    let max_length: usize = parts.iter().map(|p| p.len()).max().unwrap_or(0) + size;

    ws.iter()
        .filter(|w| {
            let word_len = w.len();
            word_len >= min_length && word_len <= max_length
        })
        .map(std::string::String::as_str)
        .collect()
}

// Count prefix occurrences for each valid word
fn count_prefix_suffix<'a>(
    valid_words: &HashSet<&'a str>,
    parts: &'a [&'a str],
    size: usize,
) -> (HashMap<&'a str, usize>, HashSet<&'a str>) {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut prefix: HashSet<&str> = HashSet::new();

    let low_parts: Vec<String> = parts.iter().map(|part| part.to_lowercase()).collect();

    for word in valid_words {
        for part in &low_parts {
            let word_len = word.len();
            let part_len = part.len();
            if part_len + size == word_len {
                if word.starts_with(part) {
                    *counts.entry(&word[word_len - size..]).or_insert(0) += 1;
                    prefix.insert(word);
                } else if word.ends_with(part) {
                    *counts.entry(&word[..size]).or_insert(0) += 1;
                }
            }
        }
    }

    (counts, prefix)
}

// Calculate solutions based on counts
fn calculate_solutions(counts: &HashMap<&str, usize>) -> String {
    counts
        .iter()
        .filter_map(|(k, v)| if *v >= 3 { Some(*k) } else { None })
        .collect::<Vec<&str>>()
        .join(" ")
        .to_uppercase()
}

// Generate words solution
fn generate_words_solution(solution: &str, parts: &[&str], prefix: &HashSet<&str>) -> Vec<String> {
    parts
        .iter()
        .map(|part| {
            let at_start = format!("{part}{solution}").to_lowercase();
            let at_end = format!("{solution}{part}").to_lowercase();

            if prefix.contains(&at_start as &str) {
                return at_start.to_uppercase();
            }

            at_end.to_uppercase()
        })
        .collect()
}

pub fn solve(puzzle: &str, size: usize, ws: &HashSet<String>) -> SolveResult {
    let parts: Vec<&str> = puzzle.split('/').map(str::trim).collect();
    let valid_words = get_valid_words(&parts, size, ws);

    let (counts, prefix) = count_prefix_suffix(&valid_words, &parts, size);
    let possible_solutions = calculate_solutions(&counts);

    let solution = if possible_solutions.contains(' ') {
        let mut solutions_list: Vec<&str> = possible_solutions.split_whitespace().collect();
        solutions_list.sort_unstable();
        solutions_list.first().map_or("", |&word| word).to_string()
    } else {
        possible_solutions
    };

    let words_solution = generate_words_solution(&solution, &parts, &prefix);

    let words_used = if words_solution.len() < 3 {
        Vec::new()
    } else {
        words_solution
    };

    SolveResult {
        solution,
        words_used,
    }
}
