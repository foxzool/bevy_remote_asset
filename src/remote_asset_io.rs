use std::path::{Path, PathBuf};
use bevy::asset::{AssetIo, AssetIoError, BoxedFuture, ChangeWatcher, Metadata};

pub struct RemoterAssetIo {
    pub(crate) default_io: Box<dyn AssetIo>,
}


fn is_http(path: &Path) -> bool {
    path.starts_with("http://") || path.starts_with("https://")
}


impl AssetIo for RemoterAssetIo {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        todo!()
    }
    fn read_directory(&self, path: &Path) -> anyhow::Result<Box<dyn Iterator<Item=PathBuf>>, AssetIoError> {
        self.default_io.read_directory(path)
    }

    fn get_metadata(&self, path: &Path) -> anyhow::Result<Metadata, AssetIoError> {
        todo!()
    }

    fn watch_path_for_changes(&self, to_watch: &Path, to_reload: Option<PathBuf>) -> anyhow::Result<(), AssetIoError> {
        todo!()
    }

    fn watch_for_changes(&self, configuration: &ChangeWatcher) -> anyhow::Result<(), AssetIoError> {
        todo!()
    }
}