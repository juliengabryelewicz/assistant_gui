use serde::{Deserialize, Serialize};
// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub weatherapi_value: String,
    pub searchcity_value: String
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    OpenFileError,
    FormatError,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    FileError,
    WriteError,
    FormatError,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("com", "JulienGabryelewicz", "Assistant")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("assistant.json");

        path
    }

    pub async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;
        let mut contents = String::new();
        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::OpenFileError)?;
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;
        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;
        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::FormatError)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::FileError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
