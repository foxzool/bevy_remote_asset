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
        if is_http(path) {
            let uri = path.to_str().unwrap();

            Box::pin(async move {
                let req = ehttp::Request::get(uri);
                let bytes = ehttp::fetch_async(req)
                    .await
                    .map_err(|_| AssetIoError::NotFound(path.to_path_buf()))?
                    .bytes;

                Ok(bytes)
            }) as _
        } else {
            self.default_io.load_path(path)
        }
    }
    fn read_directory(
        &self,
        path: &Path,
    ) -> anyhow::Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        self.default_io.read_directory(path)
    }

    fn get_metadata(&self, path: &Path) -> anyhow::Result<Metadata, AssetIoError> {
        self.default_io.get_metadata(path)
    }

    fn watch_path_for_changes(
        &self,
        to_watch: &Path,
        to_reload: Option<PathBuf>,
    ) -> anyhow::Result<(), AssetIoError> {
        if is_http(to_watch) {
            Ok(())
        } else {
            self.default_io.watch_path_for_changes(to_watch, to_reload)
        }
    }

    fn watch_for_changes(&self, configuration: &ChangeWatcher) -> anyhow::Result<(), AssetIoError> {
        self.default_io.watch_for_changes(configuration)
    }
}
