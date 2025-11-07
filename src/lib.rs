use std::path::Path;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub meta: Meta,
}

#[derive(Debug, Clone)]
pub struct Meta {
    extension: Option<String>,
    size_bytes: u64,
}

use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn f2doc(root: &Path, relative_path: &Path) -> Option<Document> {
    let path = root.join(relative_path);

    let text = match std::fs::read_to_string(&path) {
        Ok(txt) => txt,
        Err(_) => return None,
    };
    let meta = Meta {
        extension: path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_string()),
        size_bytes: path.metadata().expect("oopsie").len(),
    };

    Some(Document {
        id: relative_path.display().to_string(),
        text,
        meta,
    })
}
pub fn grab_all_documents(root: &Path) -> Vec<Document> {
    let paths: Vec<String> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let relative_path = e
                .path()
                .strip_prefix(root)
                .ok()?
                .to_string_lossy()
                .into_owned();
            Some(relative_path)
        })
        .collect();

    let docs: Vec<Document> = paths
        .par_iter()
        .filter_map(|relative_path| f2doc(root, Path::new(relative_path)))
        .collect();
    docs
}
