use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Mutex,
};

use self::fs::read_at;

#[derive(Debug)]
pub struct StoreFile {
    pub file: Mutex<BufWriter<File>>,
    pub size: u64,
}

impl StoreFile {
    pub fn new(f: File) -> Result<Self> {
        let writer = BufWriter::new(f);
        let size = writer.get_ref().metadata()?.len();

        Ok(Self {
            file: Mutex::new(writer),
            size: size,
        })
    }
}

#[derive(Debug)]
pub struct Store {
    pub store_file: StoreFile,
}

impl Store {
    pub fn new(f: File) -> Result<Self> {
        let store_file = StoreFile::new(f)?;

        Ok(Self {
            store_file: store_file,
        })
    }

    pub fn append(&mut self, data: &[u8]) -> Result<(u64, u64)> {
        let mut store_file = &mut self.store_file;

        let mut guard = match store_file.file.lock() {
            Ok(guard) => guard,
            Err(e) => return Err(anyhow!("{}", e)),
        };

        // current position is length of the file
        let position = guard.get_ref().metadata()?.len();

        let entry_size = data.len() as u64;

        guard.write(&entry_size.to_be_bytes())?;

        let written = guard.write(data)?;

        store_file.size = written as u64 + 8u64;

        Ok((written as u64, position))
    }

    pub fn read(&self, pos: u64) -> Result<Vec<u8>> {
        let mut guard = match self.store_file.file.lock() {
            Ok(guard) => guard,
            Err(e) => return Err(anyhow!("{}", e)),
        };

        guard.flush()?;

        let mut size: Vec<u8> = vec![0; 8];
        read_at(guard.get_ref(), &mut size, pos)?;

        let mut data: Vec<u8> = vec![0; size_bytes_to_usize(&size)?];
        read_at(guard.get_ref(), &mut data, pos + 8)?;

        Ok(data)
    }

    pub fn read_at(&self, buf: &mut Vec<u8>, off: i64) -> Result<isize> {
        let mut guard = match self.store_file.file.lock() {
            Ok(guard) => guard,
            Err(e) => return Err(anyhow!("{}", e)),
        };

        guard.flush()?;

        let read = read_at(guard.get_ref(), buf, off as u64)?;

        Ok(read as isize)
    }

    pub fn close(&self) -> Result<()> {
        let mut guard = match self.store_file.file.lock() {
            Ok(guard) => guard,
            Err(e) => return Err(anyhow!("{}", e)),
        };

        guard.flush()?;

        Ok(())
    }
}

fn size_bytes_to_usize(sb: &[u8]) -> Result<usize> {
    let arr: [u8; 8] = sb.try_into()?;

    let size = u64::from_be_bytes(arr);

    let size: usize = size.try_into()?;

    Ok(size)
}

pub mod fs {
    #[cfg(unix)]
    pub use unix::*;

    #[cfg(windows)]
    pub use windows::*;

    #[cfg(unix)]
    mod unix {
        use anyhow::Result;
        use std::fs::File;
        use std::os::unix::prelude::FileExt;

        pub fn read_at(file: &File, buf: &mut [u8], off: u64) -> Result<usize> {
            Ok(file.read_at(buf, off)?)
        }
    }

    #[cfg(windows)]
    mod windows {
        use anyhow::Result;
        use std::fs::File;
        use std::os::windows::prelude::FileExt;

        pub fn read_at(file: &File, buf: &mut [u8], off: u64) -> Result<usize> {
            Ok(file.seek_read(buf, off)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::{tempfile, NamedTempFile};

    #[test]
    fn test() {
        let file = NamedTempFile::new().unwrap();
        println!("{:?}", file.path());

        let x = file.keep().unwrap();

        let mut store = super::Store::new(x.0).unwrap();

        store.append("hello world".as_bytes()).unwrap();

        let r = store.read(0).unwrap();

        println!("{}", std::str::from_utf8(&r).unwrap());
    }
}
