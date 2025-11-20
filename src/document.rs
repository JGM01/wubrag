use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    path::Path,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub path: String,
    pub text: String,
    pub meta: DocumentMetadata,
}

#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    pub extension: String,
    pub size_bytes: u64,
}

static DOC_ID: AtomicU32 = AtomicU32::new(0);

pub fn grab_all_documents(root: &Path) -> Vec<Document> {
    let paths: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let binding = e.path();
            let rel = binding.strip_prefix(root).ok()?;
            Some(rel.to_string_lossy().into_owned())
        })
        .collect();

    paths
        .par_iter()
        .filter_map(|relative| load_document(root, Path::new(relative)))
        .collect()
}

fn load_document(root: &Path, relative: &Path) -> Option<Document> {
    let path = root.join(relative);
    let text = std::fs::read_to_string(&path).ok()?;

    let meta = DocumentMetadata {
        extension: path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string(),
        size_bytes: path.metadata().ok()?.len(),
    };

    Some(Document {
        id: DOC_ID.fetch_add(1, Ordering::SeqCst),
        path: relative.display().to_string(),
        text,
        meta,
    })
}
