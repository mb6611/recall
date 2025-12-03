use super::schema::default_index_path;
use super::state::IndexState;
use super::SessionIndex;
use crate::parser;
use anyhow::Result;

/// Ensure index is up-to-date before running CLI queries.
/// Discovers new/modified session files and indexes them synchronously.
/// Progress is printed to stderr.
pub fn ensure_index_fresh(index: &SessionIndex) -> Result<()> {
    // state.json lives alongside the index directory
    let index_path = default_index_path();
    let state_path = index_path
        .parent()
        .map(|p| p.join("state.json"))
        .unwrap_or_else(|| index_path.join("state.json"));

    let mut state = IndexState::load(&state_path)?;

    // Discover all session files
    let mut files = parser::discover_session_files();

    // Sort by mtime (most recent first) for better UX during indexing
    files.sort_by(|a, b| {
        let mtime_a = std::fs::metadata(a)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let mtime_b = std::fs::metadata(b)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        mtime_b.cmp(&mtime_a)
    });

    // Find files that need indexing
    let files_to_index: Vec<_> = files
        .iter()
        .filter(|f| state.needs_reindex(f))
        .cloned()
        .collect();

    let total = files_to_index.len();
    if total == 0 {
        // Nothing to index, we're fresh
        return Ok(());
    }

    eprintln!("Indexing {} session{}...", total, if total == 1 { "" } else { "s" });

    let mut writer = index.writer()?;

    for (i, file_path) in files_to_index.iter().enumerate() {
        // Delete existing documents for this file (in case of update)
        index.delete_session(&mut writer, file_path);

        // Parse and index
        match parser::parse_session_file(file_path) {
            Ok(session) => {
                if !session.messages.is_empty() {
                    let _ = index.index_session(&mut writer, &session);
                }
                // Mark as indexed even if empty (so we don't reprocess it)
                state.mark_indexed(file_path);
            }
            Err(_) => {
                // Skip failed files (they might be incomplete/corrupted)
                // Don't mark as indexed so we retry next time
            }
        }

        // Progress update every 50 files or at the end
        if (i + 1) % 50 == 0 || i + 1 == total {
            eprint!("\rIndexing {}/{}...", i + 1, total);
        }

        // Commit every 200 files to avoid memory buildup
        if (i + 1) % 200 == 0 {
            writer.commit()?;
        }
    }

    // Final commit
    writer.commit()?;
    state.save(&state_path)?;

    // Clear progress line and print completion
    eprintln!("\rIndexed {} session{}.    ", total, if total == 1 { "" } else { "s" });

    // Reload index to see new data
    index.reload()?;

    Ok(())
}
