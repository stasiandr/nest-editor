pub mod user_lib_wrapper;
pub mod user_app;

use std::{env::temp_dir, path::PathBuf};
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct UserProject {
    project_path: PathBuf,
}

impl UserProject {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project_path
        }
    }

    pub fn project_path(&self) -> &std::path::Path {
        &self.project_path
    }

    pub fn project_name(&self) -> &str {
        self.project_path.file_name().unwrap().to_str().unwrap()
    }

    pub fn absolute_project_path(&self) -> PathBuf {
        std::fs::canonicalize(&self.project_path).unwrap()
    }

    pub fn generated_dylib_path(&self) -> PathBuf {
        let project_name = self.project_name();
        self.absolute_project_path().join("target/debug/")
            .join(format!("lib{project_name}.dylib"))
    }

    pub fn copy_lib_to_temp_path(&self) -> PathBuf {
        let project_name = self.project_name();
        let uuid = Uuid::new_v4();

        let new_path = temp_dir().join(format!("lib{project_name}_{uuid}.dylib"));
        std::fs::rename(self.generated_dylib_path(), &new_path).unwrap();
        new_path
    }
}
