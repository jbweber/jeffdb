use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::RwLock,
};
pub struct StoreFile {
    pub file: BufWriter<File>,
    pub size: u64,
}

impl StoreFile {
    pub fn new(f: File) -> Self {
        let writer = BufWriter::new(f);
        let size = writer.get_ref().metadata().unwrap().len();

        Self {
            file: writer,
            size: size,
        }
    }
}

pub struct Store {
    pub store_file: RwLock<StoreFile>,
}

impl Store {
    pub fn new(f: File) -> Self {
        Self {
            store_file: RwLock::new(StoreFile::new(f)),
        }
    }

    pub fn append(&self, p: &[u8]) -> (u64, u64) {
        {
            let mut guard = self.store_file.write().unwrap();

            // position of this write is the current size
            let position = guard.size;

            // write the size of the following data block
            let write_size = u64::try_from(p.len()).unwrap();

            guard.file.write(&write_size.to_be_bytes()).unwrap();

            // write the data block
            let mut written = guard.file.write(p).unwrap();

            (0, 0)
        }
    }

    fn read(&self, pos: u64) {
        {
            let mut guard = self.store_file.write().unwrap();
            let mut file = guard.file.get_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{File, OpenOptions},
        io::BufWriter,
        io::Write,
        os::windows::prelude::FileExt,
    };

    #[test]
    fn test() {}
}
