//! Polls one branch of a local Git repository and prints each new commit as
//! an arrival, with the files it changed.
//!
//! Run `cargo run -p pipes --example git_poc <repo> [branch] [seconds]`. The
//! branch defaults to `main` and the poll interval to 5 seconds. Commit to the
//! branch while it runs, and stop it with Ctrl-C.

use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use gix::ObjectId;
use gix::object::tree::diff::ChangeDetached;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), BoxError> {
    let mut args = std::env::args().skip(1);
    let repo_path = PathBuf::from(
        args.next()
            .ok_or("usage: git_poc <repo> [branch] [seconds]")?,
    );
    let branch = args.next().unwrap_or_else(|| "main".into());
    let interval = Duration::from_secs(args.next().map_or(Ok(5), |s| s.parse())?);

    let mut last = tip(&repo_path, &branch)?;
    println!(
        "watching {} branch {branch} at {}",
        repo_path.display(),
        last.to_hex_with_len(7)
    );

    loop {
        thread::sleep(interval);
        let current = tip(&repo_path, &branch)?;
        if current != last {
            arrive(&repo_path, current, last)?;
            last = current;
        }
    }
}

/// Reads the commit id the branch points at. The repository is opened on each
/// poll, so the ref is read from disk rather than from a cache.
fn tip(repo_path: &Path, branch: &str) -> Result<ObjectId, BoxError> {
    let repo = gix::open(repo_path)?;
    let mut reference = repo.find_reference(format!("refs/heads/{branch}").as_str())?;
    Ok(reference.peel_to_commit()?.id)
}

/// Prints each commit reachable from `current` but not from `last`, oldest
/// first, with the files it changed against its first parent.
fn arrive(repo_path: &Path, current: ObjectId, last: ObjectId) -> Result<(), BoxError> {
    let repo = gix::open(repo_path)?;
    let mut ids = repo
        .rev_walk([current])
        .with_hidden([last])
        .all()?
        .map(|info| info.map(|info| info.id))
        .collect::<Result<Vec<_>, _>>()?;
    ids.reverse();

    for id in ids {
        let commit = repo.find_commit(id)?;
        let message = commit.message()?;
        println!("arrived {} {}", id.to_hex_with_len(7), message.summary());

        let tree = commit.tree()?;
        let parent_tree = match commit.parent_ids().next() {
            Some(parent) => Some(repo.find_commit(parent)?.tree()?),
            None => None,
        };
        for change in repo.diff_tree_to_tree(parent_tree.as_ref(), &tree, None)? {
            let kind = match change {
                ChangeDetached::Addition { .. } => 'A',
                ChangeDetached::Deletion { .. } => 'D',
                ChangeDetached::Modification { .. } => 'M',
                ChangeDetached::Rewrite { .. } => 'R',
            };
            println!("  {kind} {}", change.location());
        }
    }
    Ok(())
}
