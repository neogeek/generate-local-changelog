extern crate git2;
extern crate regex;
extern crate time;
extern crate version_compare;

use git2::{Error, Repository};
use regex::Regex;
use time::OffsetDateTime;
use version_compare::Version;

pub fn get_repo(path: String) -> Result<Repository, Error> {
    Repository::open(path)
}

pub fn get_remote_origin(repo: &Repository) -> String {
    let remote = repo.find_remote("origin").unwrap();

    remote.url().unwrap().to_string()
}

pub fn format_remote_origin_as_url(remote_origin: String) -> String {
    let mut url = remote_origin;

    url = Regex::new(r"^(git@|https?://)|\.git$")
        .unwrap()
        .replace_all(&url, "")
        .to_string();

    url = Regex::new(r":").unwrap().replace_all(&url, "/").to_string();

    format!("https://{}", url)
}

pub fn get_repo_tags(repo: &Repository) -> Vec<String> {
    let mut tags = Vec::new();

    for reference in repo.references().unwrap().filter_map(Result::ok) {
        if reference.is_tag() {
            let tag_name = reference.shorthand().unwrap();

            tags.push(tag_name.to_string());
        }
    }

    tags.sort_by(|a, b| {
        let a = Version::from(a).unwrap();
        let b = Version::from(b).unwrap();

        a.partial_cmp(&b).unwrap()
    });

    tags.reverse();

    tags
}

pub fn get_tag_date(repo: &Repository, tag: &str) -> String {
    let tag_ref = repo.revparse_single(&format_reference(tag)).unwrap();

    let tag_commit = tag_ref.peel_to_commit().unwrap();

    let timestamp = tag_commit.time();

    let offset_seconds = timestamp.offset_minutes() as i64 * 60;

    let seconds = timestamp.seconds() + offset_seconds;

    let time = OffsetDateTime::from_unix_timestamp(seconds).unwrap();

    format!(
        "{}-{:0>2}-{:0>2}",
        time.year(),
        time.month() as i32,
        time.day()
    )
}

pub fn format_reference(tag: &str) -> String {
    if tag == "HEAD" {
        return tag.to_string();
    }

    format!("refs/tags/{}", tag)
}

pub fn get_merges_between_tags(
    repo: &Repository,
    tag_b: &str,
    tag_a: &str,
    incomplete: bool,
) -> Vec<String> {
    let remote_origin = get_remote_origin(repo);

    let git_url = format_remote_origin_as_url(remote_origin);

    let mut merges = Vec::new();

    let tag_a_ref = repo.refname_to_id(&format_reference(tag_a)).unwrap();
    let tag_b_ref = repo.refname_to_id(&format_reference(tag_b)).unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.hide(tag_b_ref).unwrap();
    revwalk.push(tag_a_ref).unwrap();

    let pattern = Regex::new(r"#([0-9]+)").unwrap();

    for oid in revwalk {
        let commit = repo.find_commit(oid.unwrap()).unwrap();

        let commit_message = commit.message().unwrap_or("No message");

        if (incomplete || commit.parent_count() > 1) && pattern.is_match(commit_message) {
            let pr_number = Regex::new(r"#([0-9]+)")
                .unwrap()
                .captures(commit_message)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            let commit_message_filtered: Vec<&str> = commit_message
                .lines()
                .filter(|line| !line.trim().is_empty())
                .filter(|line| !line.trim().starts_with("Merge pull request"))
                .collect();

            merges.push(format!(
                "- {} [#{}]({}/pull/{})",
                commit_message_filtered.first().unwrap().to_string().trim(),
                pr_number,
                git_url,
                pr_number
            ))
        }
    }

    merges
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_repo() {
        let repo = crate::git::get_repo("./".to_string()).unwrap();

        assert!(!repo.is_empty().unwrap());
    }
    #[test]
    fn get_remote_origin() {
        let repo = crate::git::get_repo("./".to_string()).unwrap();

        let remote_origin = crate::git::get_remote_origin(&repo);

        let is_http_url = remote_origin == "https://github.com/neogeek/generate-local-changelog";

        let is_git_url = remote_origin == "git@github.com:neogeek/generate-local-changelog.git";

        assert!(is_http_url || is_git_url);
    }
    #[test]
    fn format_remote_origin_as_url() {
        assert_eq!(
            crate::git::format_remote_origin_as_url(
                "git@github.com:neogeek/generate-local-changelog.git".to_string()
            ),
            "https://github.com/neogeek/generate-local-changelog"
        );
    }
    #[test]
    fn format_reference_head() {
        assert_eq!(crate::git::format_reference("HEAD"), "HEAD");
    }
    #[test]
    fn format_reference_tag() {
        assert_eq!(crate::git::format_reference("v1.0.0"), "refs/tags/v1.0.0");
    }
}
