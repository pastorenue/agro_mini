use std::{
    fs::{self, File},
    io,
    path::Path,
};


pub(crate) struct LocalFs {}

impl LocalFs {
    pub fn options() -> OpenOptions {
        OpenOptions::default()
    }
}

pub(crate) trait FsBlockSize {
     /// Returns the underlying file system block size or preferred IO buffer
    /// size.
    fn block_size(path: impl AsRef<Path>) -> io::Result<usize>;
}

pub(crate) trait Open {
    /// Opens the file with the specified [`OpenOptions`].
    fn open(self, path: impl AsRef<Path>) -> io::Result<File>;
}

pub(crate) struct OpenOptions {
    /// Instance of [`std::fs::OpenOptions`]
    inner: fs::OpenOptions,
    /// Bypasses OS cache
    bypass_cache: bool,
    sync_on_write: bool,
    lock: bool,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            inner: File::options(),
            bypass_cache: false,
            sync_on_write: false,
            lock: false,
        }
    }
}

impl OpenOptions {
    pub fn bypass_cache(mut self, bypass_cache: bool) -> Self {
        self.bypass_cache = bypass_cache;
        self
    }

    pub fn sync_on_write(mut self, sync_on_write: bool) -> Self {
        self.sync_on_write = sync_on_write;
        self
    }

    pub fn lock(mut self, lock: bool) -> Self {
        self.lock = lock;
        self
    }

    pub fn create(mut self, create: bool) -> Self {
        self.inner.create(create);
        self
    }

    /// Set the length of the file to 0 bytes if it exists.
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.inner.truncate(truncate);
        self
    }

    pub fn read(mut self, read: bool) -> Self {
        self.inner.read(read);
        self
    }

    pub fn write(mut self, write: bool) -> Self {
        self.inner.write(write);
        self
    }

}

#[cfg(unix)]
mod unix {
    use std::{
        fs::File,
        io,
        os::{
            fd::AsRawFd,
            unix::{fs::OpenOptionsExt, prelude::MetadataExt},
        },
        path::Path,
    };

    use super::{LocalFs, OpenOptions, Open, FsBlockSize};

    impl FsBlockSize for LocalFs {
        fn block_size(path: impl AsRef<Path>) -> io::Result<usize> {
            Ok(File::open(path)?.metadata()?.blksize() as usize)
        }
    }

    impl Open for OpenOptions {
        fn open(mut self, path: impl AsRef<Path>) -> io::Result<File> {
            let mut flags = 0;

            if self.sync_on_write {
                flags |= libc::O_SYNC;
            }

            if flags != 0 {
                self.inner
                    .custom_flags(flags);
            }

            let file = self.inner.open(path)?;

            if self.lock {
                let lock = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
                if lock != 0 {
                    return Err(io::Error::last_os_error());
                }
            }

            Ok(file)
        }

    }
}

#[cfg(windows)]
mod windows {
    use std::{
        fs::File,
        io,
        os::windows::{ffi::OsStrExt, fs::OpenOptionsExt},
        path::Path,
    };

    use windows::{
        core::PCWSTR,
        Win32::{Foundation::MAX_PATH, Storage::FileSystem},
    };

    use super::{FileSystemBlockSize, Fs, Open, OpenOptions};

    impl FileSystemBlockSize for Fs {
        fn block_size(path: impl AsRef<Path>) -> io::Result<usize> {
            unsafe {
                let mut volume = [0u16; MAX_PATH as usize];

                let mut windows_file_path = path
                    .as_ref()
                    .as_os_str()
                    .encode_wide()
                    .collect::<Vec<u16>>();

                // encode_wide() does not add the null terminator.
                windows_file_path.push(0);

                FileSystem::GetVolumePathNameW(
                    PCWSTR::from_raw(windows_file_path.as_ptr()),
                    &mut volume,
                )?;

                let mut bytes_per_sector: u32 = 0;
                let mut sectors_per_cluster: u32 = 0;

                FileSystem::GetDiskFreeSpaceW(
                    PCWSTR::from_raw(volume.as_ptr()),
                    Some(&mut bytes_per_sector),
                    Some(&mut sectors_per_cluster),
                    None,
                    None,
                )?;

                Ok((bytes_per_sector * sectors_per_cluster) as usize)
            }
        }
    }

    impl Open for OpenOptions {
        fn open(mut self, path: impl AsRef<Path>) -> io::Result<File> {
            let mut flags = FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0);

            if self.bypass_cache {
                flags |= FileSystem::FILE_FLAG_NO_BUFFERING;
            }

            if self.sync_on_write {
                flags |= FileSystem::FILE_FLAG_WRITE_THROUGH;
            }

            if flags.0 != 0 {
                self.inner.custom_flags(flags.0);
            }

            if self.lock {
                self.inner.share_mode(0);
            }

            self.inner.open(path)
        }
    }
}
