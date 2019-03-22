// NOTE: Run with `cargo test` rather than `cargo run`

// O(n^3) solutions are fine

// Part 1: substring indices

#![crate_type = "lib"]

/// Find indicies into `text` which point to the start of substrings equal to `subject`
pub fn find_substring_indicies(text: &str, subject: &str) -> Vec<usize> {
    let mut indices = Vec::new();

    for i in 0..(text.len() - subject.len() + 1) {
        let text_slice = &text[i..i+subject.len()];
        if text_slice == subject {  // Comparing two slices with == actually checks the values
            indices.push(i);
        }
    }

    indices
}

#[test]
fn simple_test1() {
    let text = "ababacaba";
    let subject = "ab";
    let result = find_substring_indicies(text, subject);
    assert_eq!(result, vec![0, 2, 6]);
}

#[test]
fn simple_test2() {
    let text = "aaabaa";
    let subject = "aa";
    let result = find_substring_indicies(text, subject);
    assert_eq!(result, vec![0, 1, 4]);
}

// Part 2: anagrams

/// Find indicies into `text` which point to the start of anagrams of `subject`
pub fn find_anagram_indicies(text: &str, subject: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut subject_sorted: Vec<char> = subject.chars().collect();
    subject_sorted.sort();

    for i in 0..(text.len() - subject.len() + 1) {
        let text_slice = &text[i..i+subject.len()];
        let mut text_slice_sorted: Vec<char> = text_slice.chars().collect();
        text_slice_sorted.sort();
        if text_slice_sorted == subject_sorted {
            indices.push(i);
        }
    }

    indices
}

#[test]
fn simple_test3() {
    let text = "cbaebabacd";
    let subject = "abc";
    let result = find_anagram_indicies(text, subject);
    assert_eq!(result, vec![0, 6]);
}

#[test]
fn simple_test4() {
    let text = "acbaebabacd";
    let subject = "abc";
    let result = find_anagram_indicies(text, subject);
    assert_eq!(result, vec![0, 1, 7]);
}

