//! `SourceCodeInfo` comment lookup.

use crate::descriptor::FileDescriptorProto;
use buffa_descriptor::generated::descriptor::source_code_info::Location;
use std::collections::HashMap;

pub mod path {
    pub const FILE_MESSAGE: i32 = 4;
    pub const FILE_ENUM: i32 = 5;
    pub const FILE_SERVICE: i32 = 6;
    pub const MSG_FIELD: i32 = 2;
    pub const MSG_ONEOF: i32 = 8;
    pub const MSG_OPTIONS: i32 = 7;
    pub const ENUM_VALUE: i32 = 2;
    pub const SVC_METHOD: i32 = 2;
}

pub struct CommentIndex<'a> {
    by_path: HashMap<Vec<i32>, &'a Location>,
}

impl<'a> CommentIndex<'a> {
    pub fn from_file(file: &'a FileDescriptorProto) -> Self {
        let mut by_path = HashMap::new();
        if let Some(info) = file.source_code_info.as_option() {
            for loc in &info.location {
                if !loc.path.is_empty() {
                    by_path.insert(loc.path.clone(), loc);
                }
            }
        }
        Self { by_path }
    }

    pub fn leading(&self, path: &[i32]) -> Option<&str> {
        self.by_path
            .get(path)
            .and_then(|l| l.leading_comments.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }

    pub fn span_snippet(&self, source: &str, path: &[i32]) -> Option<String> {
        let loc = self.by_path.get(path)?;
        if loc.span.is_empty() {
            return None;
        }
        crate::populate::source::extract_span_snippet(source, &loc.span)
    }

    pub fn leading_message(&self, mi: usize) -> Option<&str> {
        self.leading(&[path::FILE_MESSAGE, mi as i32])
    }

    pub fn leading_message_field(&self, mi: usize, fi: usize) -> Option<&str> {
        self.leading(&[path::FILE_MESSAGE, mi as i32, path::MSG_FIELD, fi as i32])
    }

    pub fn leading_enum(&self, ei: usize) -> Option<&str> {
        self.leading(&[path::FILE_ENUM, ei as i32])
    }

    pub fn leading_enum_value(&self, ei: usize, vi: usize) -> Option<&str> {
        self.leading(&[path::FILE_ENUM, ei as i32, path::ENUM_VALUE, vi as i32])
    }

    pub fn leading_service(&self, si: usize) -> Option<&str> {
        self.leading(&[path::FILE_SERVICE, si as i32])
    }

    pub fn leading_method(&self, si: usize, mi: usize) -> Option<&str> {
        self.leading(&[path::FILE_SERVICE, si as i32, path::SVC_METHOD, mi as i32])
    }
}

pub fn package_overview(files: &[(&str, &FileDescriptorProto)]) -> Option<String> {
    let mut sorted = files.to_vec();
    sorted.sort_by_key(|(name, _)| *name);
    for (_, file) in sorted {
        let idx = CommentIndex::from_file(file);
        if let Some(c) = idx.leading(&[2]) {
            return Some(c.to_string());
        }
    }
    None
}

pub fn dedent_comment(comment: &str) -> String {
    let lines: Vec<&str> = comment.trim().lines().collect();
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|l| {
            if l.trim().is_empty() {
                String::new()
            } else {
                l.chars().skip(min_indent).collect()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
