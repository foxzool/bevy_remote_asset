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

            #[cfg(target_arch = "wasm32")]
                let fut = Box::pin(async move {
                use wasm_bindgen::JsCast;
                use wasm_bindgen_futures::JsFuture;
                let window = web_sys::window().unwrap();
                let response = JsFuture::from(window.fetch_with_str(uri))
                    .await
                    .map(|r| r.dyn_into::<web_sys::Response>().unwrap())
                    .map_err(|e| e.dyn_into::<js_sys::TypeError>().unwrap());

                if let Err(err) = &response {
                    // warn!("Failed to fetch asset {uri}: {err:?}");
                }

                let response = response.map_err(|_| AssetIoError::NotFound(path.to_path_buf()))?;

                let data = JsFuture::from(response.array_buffer().unwrap())
                    .await
                    .unwrap();

                let bytes = js_sys::Uint8Array::new(&data).to_vec();

                Ok(bytes)
            });

            #[cfg(not(target_arch = "wasm32"))]
                let fut = Box::pin(async move {
                // let bytes = async_compat::Compat::new(async {
                //     reqwest::get(uri).await.map_err(|_| AssetIoError::NotFound(path.to_path_buf()))?
                //         .bytes()
                //         .await
                //         .map_err(|_| AssetIoError::NotFound(path.to_path_buf()))?;
                // });
                let bytes = reqwest::get(uri).await.map_err(|_| AssetIoError::NotFound(path.to_path_buf()))?
                    .bytes()
                    .await
                    .map_err(|_| AssetIoError::NotFound(path.to_path_buf()))?.to_vec();

                Ok(bytes)
            });

            fut
        } else {
            self.default_io.load_path(path)
        }
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