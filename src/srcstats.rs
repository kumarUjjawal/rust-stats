use super::errors::StatsError;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SrcStats {
    pub number_of_files: u32,
    pub loc: u32,
    pub comments: u32,
    pub blanks: u32,
}

pub fn get_summary_src_stats(in_dir: &Path) -> Result<SrcStats, StatsError> {
    let mut total_loc = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;
    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut file_entries: Vec<DirEntry> = vec![];

    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().is_dir() {
                    dir_entries.push(entry.path());
                } else {
                    if entry.path().extension() == Some(OsStr::new("rs")) {
                        file_entries.push(entry);
                    }
                }
            }
        }
    }

    let file_count = file_entries.len();
    for entry in file_entries {
        let stat = get_src_stats_for_file(&entry.path())?;
        total_loc += stat.loc;
        total_blanks += stat.blanks;
        total_comments += stat.comments;
    }

    Ok(SrcStats {
        number_of_files: u32::try_from(file_count)?,
        loc: total_loc,
        comments: total_comments,
        blanks: total_blanks,
    })
}
