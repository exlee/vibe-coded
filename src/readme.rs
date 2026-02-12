use std::path::{PathBuf};
use unicode_segmentation::UnicodeSegmentation;
use pulldown_cmark::{Parser, Event, Tag};

use crate::traits::Repo;

pub fn get_readme(repo: &Repo) -> Option<String>{
    let blob = repo
        .head().ok()?
        .peel_to_tree().ok()?
        .get_path(&PathBuf::from("README.md")).ok()?
        .to_object(repo).ok()?
        .peel_to_blob().ok()?
        ;
        String::from_utf8(blob.content().to_vec()).ok()

}

pub fn count_emojis(input: &str) -> usize {
    input.graphemes(true)
    .filter(|g| {
                // A grapheme is an emoji if any of its chars fall in emoji ranges
                g.chars().any(|c| matches!(c, 
                    '\u{1F300}'..='\u{1F9FF}' | 
                    '\u{1FA00}'..='\u{1FAFF}' |
                    '\u{2600}'..='\u{26FF}'   |
                    '\u{2700}'..='\u{27BF}'
                ))
            })
            .count()
}

pub fn count_headings_from_repo(repo: &Repo) -> Option<usize> {
    get_readme(repo).map(|s| count_headings(&s))
}
pub fn count_headings(markdown: &str) -> usize {
    let parser = Parser::new(markdown);
    
    parser.filter(|event| {
        matches!(event, Event::Start(Tag::Heading { .. }))
    })
    .count()
}

const LLM_INDICATORS: [&str;25] = [
    "delve",
    "tapestry",
    "realm",
    "testament",
    "underscore",
    "harness",
    "leverage",
    "meticulous",
    "elevate",
    "foster",
    "unleash",
    "comprehensive",
    "transformative",
    "demystify",
    "paradigm",
    "showcase",
    "intricacies",
    "pivotal",
    "multifaceted",
    "ever-evolving",
    "landscape",
    "dynamic",
    "robust",
    "spearhead",
    "symphony",
];

pub fn count_llm_words(markdown: &str) -> usize {
    let mut count = 0;
    for indicator in LLM_INDICATORS {
        let mut saw = false;
        for _ in markdown.matches(indicator) {
            count += 1;
            if !saw {
                saw = true;
                //println!("{}", indicator);
            }

        }
    }
    count
}
pub fn count_llm_words_repo(repo: &Repo) -> Option<usize> {
    let readme = crate::readme::get_readme(repo)?;
    Some(crate::readme::count_llm_words(&readme))
}
