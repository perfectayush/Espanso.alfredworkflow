//! Alfred Script Filter: fuzzy-search espanso snippets with skim's matching engine.

use std::path::PathBuf;
use std::process::Command;
use std::{env, iter};

use powerpack::Item;
use serde::Deserialize;
use skim::item::RankBuilder;
use skim::prelude::{AndOrEngineFactory, ExactOrFuzzyEngineFactory};
use skim::MatchEngineFactory;

#[derive(Deserialize)]
struct Snippet {
    triggers: Vec<String>,
    replace: String,
    label: Option<String>,
}

impl Snippet {
    /// The text that queries are matched against: triggers, label, and
    /// the replacement with whitespace collapsed.
    fn haystack(&self) -> String {
        let replace = self.replace.split_whitespace().collect::<Vec<_>>().join(" ");
        let label = self.label.as_deref().unwrap_or_default();
        format!("{} {label} {replace}", self.triggers.join(" "))
    }

    fn item(&self) -> Item {
        Item::new(&self.triggers[0])
            .subtitle(self.label.as_deref().unwrap_or_default())
            .arg(&self.triggers[0])
            .large_type_text(&self.replace)
    }
}

/// Returns the indices of `haystacks` matching `query` (fzf extended-search
/// syntax), best match first; ties keep their original order.
fn search(haystacks: &[String], query: &str) -> Vec<usize> {
    let engine = AndOrEngineFactory::new(ExactOrFuzzyEngineFactory::builder().filter_mode(true).build())
        .create_engine(query);
    let criteria = RankBuilder::default();
    let mut hits: Vec<_> = haystacks
        .iter()
        .enumerate()
        .filter_map(|(i, text)| {
            let rank = engine.match_item(text)?.rank;
            Some((rank.sort_key(criteria.criteria()), i))
        })
        .collect();
    hits.sort(); // sort keys put the best match first; the index breaks ties
    hits.into_iter().map(|(_, i)| i).collect()
}

fn espanso_path() -> PathBuf {
    let path = env::var_os("PATH").unwrap_or_default();
    // Alfred runs with a minimal PATH, so also look in the Homebrew locations.
    env::split_paths(&path)
        .chain(["/opt/homebrew/bin", "/usr/local/bin"].map(PathBuf::from))
        .map(|dir| dir.join("espanso"))
        .find(|p| p.is_file())
        .unwrap_or_else(|| PathBuf::from("espanso"))
}

fn load_snippets() -> Result<Vec<Snippet>, String> {
    let out = Command::new(espanso_path())
        .args(["match", "list", "--json"])
        .output()
        .map_err(|e| format!("could not run espanso: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("espanso failed ({}): {}", out.status, stderr.trim()));
    }
    let snippets: Vec<Snippet> =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("invalid espanso output: {e}"))?;
    Ok(snippets.into_iter().filter(|s| !s.triggers.is_empty()).collect())
}

fn main() {
    let query = env::args().nth(1).unwrap_or_default();
    let result = load_snippets().map(|snippets| {
        let haystacks: Vec<_> = snippets.iter().map(Snippet::haystack).collect();
        search(&haystacks, query.trim())
            .into_iter()
            .map(|i| snippets[i].item())
            .collect::<Vec<_>>()
    });
    let written = match result {
        Ok(items) => powerpack::output(items),
        Err(msg) => powerpack::output(iter::once(
            Item::new("Espanso search failed").subtitle(msg).valid(false),
        )),
    };
    if let Err(e) = written {
        eprintln!("failed to write Alfred output: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hays() -> Vec<String> {
        ["date today", "email me@example.com", "espanso help", "lorem ipsum", "dated entry"]
            .map(String::from)
            .to_vec()
    }

    #[test]
    fn empty_query_returns_all_in_order() {
        assert_eq!(search(&hays(), ""), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn fuzzy_term() {
        assert_eq!(search(&hays(), "lrm"), vec![3]);
    }

    #[test]
    fn exact_term() {
        assert_eq!(search(&hays(), "'esp"), vec![2]);
        assert!(search(&hays(), "'lrm").is_empty());
    }

    #[test]
    fn multiple_terms_are_anded() {
        assert_eq!(search(&hays(), "date entry"), vec![4]);
        assert_eq!(search(&hays(), "date today"), vec![0]);
    }

    #[test]
    fn inverse_term() {
        assert_eq!(search(&hays(), "!date"), vec![1, 2, 3]);
    }
}
