/*
    Compute Levenshtein distance between two strings.

    Copied from wikipedia, which admits its inefficient, but simple.

    https://en.wikipedia.org/wiki/Levenshtein_distance

    'easy' way would to be cache results in map and reuse for the entirety of recursion
*/
pub fn levenshtein(a: &String, b: &String) -> u32 {
    if a.len() == 0 {
        return b.len().try_into().expect("no convert");
    }

    if (b.len() == 0) {
        return a.len().try_into().expect("no convert");
    }

    let head_a = a.chars().next();
    let head_b = b.chars().next();

    let tail_a = String::from(&a[1..]);
    let tail_b = String::from(&b[1..]);

    if head_a == head_b {
        return levenshtein(&tail_a, &tail_b);
    }

    return 1 + [
        // head of b prepended to a
        levenshtein(&a, &tail_b),
        // head of a deleted
        levenshtein(&tail_a, &b),
        // head of a replaced by head of b
        levenshtein(&tail_a, &tail_b)
    ].iter().min().unwrap();
}