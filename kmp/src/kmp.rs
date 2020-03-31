/// Use the [Knuth-Morris-Pratt](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm)
/// string matching algorithm to find all occurrences of `word` in `body`.
///
/// # Return
///     A vector of start positions for all occurrences.
fn string_match(word: &str, body: &str) -> Vec<usize> {
    // Shortcut
    if body.len() < word.len() {
        return Vec::new()
    }

    let mut offsets: Vec<usize> = Vec::new();

    let partial_match_table = prefix_function(word);

    let mut k: i32 = 0;
    let mut j: usize = 0;

    while j < body.len() {
        let c = char_at(body, j);
       if char_at(word, k as usize) == c  {
           j = j + 1;
           k = k + 1;
           if k == word.len() as i32 {
               offsets.push(j - k as usize) ;
               k = partial_match_table[word.len()];
           }
       } else {
           k = partial_match_table[k as usize];
           if k < 0 {
               j = j + 1;
               k = k + 1
           }
       }
    }
    offsets
}

fn prefix_function(word: &str) -> Vec<i32> {
    let mut table: Vec<i32> = vec![0; word.len() + 1];

    table[0] = -1;

    let mut cnd: i32 = 0;
    let mut pos: i32 = 1;

    while (pos as usize) < word.len() {
       if char_at(word, pos as usize) == char_at(word, cnd as usize) {
          table[pos as usize] = table[cnd as usize];
       }  else {
           table[pos as usize] = cnd;
           cnd = table[cnd as usize];
           while cnd >= 0 && char_at(word, pos as usize) != char_at(word, cnd as usize) {
               cnd = table[cnd as usize];
           }
       }
        pos = pos + 1;
        cnd = cnd + 1;
    }

    table[pos as usize] = cnd;

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
    use crate::kmp::{string_match, prefix_function};

    #[test]
    fn prefix_match_table() {
        let result = prefix_function("ABCDABD") ;

        assert_eq!(result, vec![-1, 0, 0, 0, -1, 0, 2, 0])
    }

    #[test]
    fn kmp_match() {
        let result = string_match("ABCDABD", "ABC ABCDAB ABCDABCDABDE");

        assert_eq!(result, vec![15])
    }
}