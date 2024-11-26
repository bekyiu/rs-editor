use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct FileInfo {
    // 文件路径
    pub path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(filename: &str) -> Self {
        Self {
            path: Some(PathBuf::from(filename)),
        }
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.path.as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");

        write!(f, "{}", name)
    }
}