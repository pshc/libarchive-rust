use std::default::Default;
use std::ffi::CString;
use std::path::Path;

use libarchive3_sys::ffi;

use archive::Handle;
use error::{ArchiveResult, ArchiveError};
use super::{ArchiveHandle, Builder, Reader, ReaderEntry};

const BLOCK_SIZE: usize = 10240;

pub struct FileReader {
    handle: ArchiveHandle,
    entry: ReaderEntry,
}

impl FileReader {
    pub fn open<T: AsRef<Path>>(builder: Builder, file: T) -> ArchiveResult<Self> {
        let c_file = CString::new(file.as_ref().to_string_lossy().as_bytes()).unwrap();
        unsafe {
            match ffi::archive_read_open_filename(builder.handle(), c_file.as_ptr(), BLOCK_SIZE) {
                ffi::ARCHIVE_OK => {
                    Ok(Self::new(builder.into()))
                }
                _ => Err(ArchiveError::from(&builder as &Handle)),
            }
        }
    }

    /// Opens archive backed by given file descriptor.
    /// Note that the file descriptor is not owned, i.e. it won't be closed
    /// on destruction of FileReader.
    /// It's your responsibility to close the descriptor after it's no longer used by FileReader.
    /// This is hinted at by taking RawFd by reference.
    #[cfg(unix)]
    pub fn open_fd(builder: Builder, fd: &::std::os::unix::io::RawFd) -> ArchiveResult<Self> {
        unsafe {
            match ffi::archive_read_open_fd(builder.handle(), *fd, BLOCK_SIZE) {
                ffi::ARCHIVE_OK => {
                    Ok(Self::new(builder.into()))
                }
                _ => Err(ArchiveError::from(&builder as &Handle)),
            }
        }
    }

    fn new(handle: ArchiveHandle) -> Self {
        FileReader {
            handle: handle,
            entry: ReaderEntry::default(),
        }
    }
}

impl Handle for FileReader {
    unsafe fn handle(&self) -> &mut ffi::Struct_archive {
        self.handle.handle()
    }
}

impl Reader for FileReader {
    fn entry(&mut self) -> &mut ReaderEntry {
        &mut self.entry
    }
}