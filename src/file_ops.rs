use std::path::Path;
use crate::os::{Open, FsBlockSize, OpenOptions};
use std::fs::File;


pub(crate) fn read_file(path: impl AsRef<Path>) -> std::io::Result<File> {
    let file = crate::os::LocalFs::options()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .bypass_cache(true)
        .sync_on_write(true)
        .lock(true)
        .open(path)?;

    file.sync_all();

    Ok(file)
}