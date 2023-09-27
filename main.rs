// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::borrow::Cow;

fn main() {
    prefix_matches("/v1/publishers", "/v1/publishersBooks");
}

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let matches: bool;
    let wild_string = insert_wildcard(prefix, request_path);
    matches = if prefix.len() > wild_string.len() {
        false
    } else {
        if prefix.len() == wild_string.len() {
            println!("comparison {} {}", add_slash(&prefix), wild_string);
            add_slash(&prefix) == add_slash(&wild_string[0..prefix.len()])
        } else {
            println!(
                "comparison {} {}",
                add_slash(&prefix),
                &wild_string[0..prefix.len() + 1]
            );
            add_slash(&prefix) == &wild_string[0..prefix.len() + 1]
        }
    };

    matches
}

fn add_slash(unslashed: &str) -> Cow<str> {
    let mut longer = unslashed.to_string();
    longer.push('/');
    Cow::Owned(longer)
}

fn insert_wildcard<'a>(short_string: &'a str, full_string: &'a str) -> Cow<'a, str> {
    let to_shorten = full_string.to_string();
    if let Some(pos) = short_string.find('*') {
        if short_string <= full_string {
            println!("short string {}", short_string);
            println!("to shorten {}", to_shorten);
            let first_half = &to_shorten[0..pos];
            let second_half = &to_shorten[pos..to_shorten.len()];
            let mut rest = "";
            if let Some(rpos) = second_half.find('/') {
                rest = &second_half[rpos..second_half.len()];
            } else {
            }
            //println!("original {}", full_string);
            //println!("halves {} {}", first_half, second_half);
            //println!("rest {}", rest);

            let answer = format!("{}{}{}", &first_half, "*", rest);
            println!("answer {}", answer);

            Cow::Owned(answer)
        } else {
            Cow::Owned(to_shorten)
        }
    } else {
        Cow::Owned(to_shorten)
    }
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
