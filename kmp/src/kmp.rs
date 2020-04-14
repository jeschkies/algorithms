/// Use the [Knuth-Morris-Pratt](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm)
/// string matching algorithm to find all occurrences of `word` in `body`.
///
/// # Return
/// A vector of start positions for all occurrences.
fn string_match(word: &str, body: &str) -> Vec<usize> {
    // Shortcut
    if word.is_empty() || body.len() < word.len() {
        return Vec::new();
    }

    let mut offsets: Vec<usize> = Vec::new();

    let partial_match_table = prefix_function(word);

    let mut q: i32 = -1;

    for (i, c) in body.chars().enumerate() {
        while q >= 0 && char_at(word, q as usize + 1) != c {
            q = partial_match_table[q as usize];
        }

        if char_at(word, (q + 1) as usize) == c {
            q = q + 1;
        }

        if (q + 1) as usize == word.len() {
            offsets.push(i - q as usize);
            q = partial_match_table[q as usize];
        }
    }

    offsets
}

fn prefix_function(word: &str) -> Vec<i32> {
    let mut table: Vec<i32> = vec![0; word.len()];

    table[0] = 0;

    let mut k: i32 = -1;

    for (q, c) in word.chars().enumerate().skip(1) {
        while k >= 0 && char_at(word, (k + 1) as usize) != c {
            k = table[k as usize];
        }

        if char_at(word, (k + 1) as usize) == c {
            k = k + 1;
        }

        table[q] = k;
    }

    table
}

/// Return the character at the given position or panic if we are out of bound.
///
/// This method is copied from the [Rust Regex parse](https://github.com/rust-lang/regex/blob/master/regex-syntax/src/ast/parse.rs#L461).
fn char_at(s: &str, i: usize) -> char {
    s[i..].chars().next().expect("Index out of bound.")
}

#[cfg(test)]
mod tests {
    use crate::kmp::{prefix_function, string_match};

    #[test]
    fn prefix_match_table() {
        assert_eq!(prefix_function("ABCDABD"), vec![0, -1, -1, -1, 0, 1, -1]);
        assert_eq!(prefix_function("AAAA"), vec![0, 0, 1, 2]);
        assert_eq!(prefix_function("ABAB"), vec![0, -1, 0, 1]);
    }

    #[test]
    fn kmp_match() {
        let result = string_match("ABCDABD", "ABC ABCDAB ABCDABCDABDABCDABDE");
        assert_eq!(result, vec![15, 22]);

        assert_eq!(string_match("ABAB", "aaaABCABABcc"), vec![6]);
    }
}
