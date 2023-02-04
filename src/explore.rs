use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Read, Write},
    os::windows::prelude::FileExt,
};

use anyhow::Result;

fn main() -> Result<()> {
    fs::remove_file("C:\\users\\jweber\\test.txt")?;

    let f = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .write(true)
        .open("C:\\users\\jweber\\test.txt")
        .unwrap();
    let mut w: BufWriter<File> = BufWriter::new(f);

    for i in 0..8 {
        let to_write = u64::from(i + 1 as u32).to_be_bytes();
        w.write(&to_write)?;
    }

    w.flush()?;

    println!("===========================================");

    for i in 0..8 {
        let mut v: Vec<u8> = vec![0; 8];
        let read_bytes = read_at(w.get_ref(), &mut v, i * 8)?;
        assert_eq!(read_bytes, 8);
        print_it(&v);
    }

    println!("===========================================");

    for i in 0..4 {
        let mut v: Vec<u8> = vec![0; 8];
        let read_bytes = read_at(w.get_ref(), &mut v, i * 8)?;
        assert_eq!(read_bytes, 8);
        print_it(&v);
    }

    println!("===========================================");

    let mut buf: Vec<u8> = vec![0; 8];
    let s = w.get_ref().read(&mut buf)?;
    println!("{}", s);
    print_it(&buf);

    for i in 8..16 {
        let to_write = u64::from(i + 1 as u32).to_be_bytes();
        w.write(&to_write)?;
    }

    w.flush()?;

    println!("===========================================");

    for i in 0..16 {
        let mut v: Vec<u8> = vec![0; 8];
        let read_bytes = read_at(w.get_ref(), &mut v, i * 8)?;
        assert_eq!(read_bytes, 8);
        print_it(&v);
    }

    println!("===========================================");

    // let one = 1u64.to_be_bytes();
    // let two: u64 = 2;
    // let three: u64 = 3;
    // let four: u64 = 4;

    // let one_w = w.write(&one).unwrap();
    // assert_eq!(one_w, 8);

    // let two_w = w.write(&two.to_be_bytes()).unwrap();
    // assert_eq!(two_w, 8);

    // w.flush()?;

    // let mut v: Vec<u8> = vec![0; 8];

    // let read_bytes = read_at(w.get_ref(), &mut v, 0)?;
    // assert_eq!(read_bytes, 8);
    // print_it(&v);

    // let three_w = w.write(&three.to_be_bytes()).unwrap();
    // assert_eq!(three_w, 8);

    // let four_w = w.write(&four.to_be_bytes()).unwrap();
    // assert_eq!(four_w, 8);

    // w.flush()?;

    // let read_bytes = read_at(w.get_ref(), &mut v, 8)?;
    // assert_eq!(read_bytes, 8);
    // print_it(&v);

    // let read_bytes = read_at(w.get_ref(), &mut v, 16)?;
    // assert_eq!(read_bytes, 8);
    // print_it(&v);

    // let read_bytes = read_at(w.get_ref(), &mut v, 24)?;
    // assert_eq!(read_bytes, 8);
    // print_it(&v);

    Ok(())
}

fn toa(sl: &[u8]) -> [u8; 8] {
    sl.try_into().expect("slice with incorrect length")
}

fn print_it(it: &[u8]) {
    println!("{:?}", it);
    let v = u64::from_be_bytes(toa(it));
    println!("{}", v);
}

fn read_at(f: &File, buf: &mut [u8], off: u64) -> Result<usize> {
    let z = f.seek_read(buf, off)?;

    Ok(z)
}
