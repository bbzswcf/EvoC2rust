use crate::translation_utils::*;

use core::ops::*;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::ptr::NonNull;

pub struct FileInfo {
    pub byte_buf: u8,
    pub buf_unread: bool,
}

pub struct FilePtr {
    pub file: Option<NonNull<File>>,
    pub info: Option<NonNull<FileInfo>>,
}

impl Copy for FilePtr {}

impl Clone for FilePtr {
    fn clone(&self) -> Self {
        FilePtr {
            file: self.file,
            info: self.info,
        }
    }
}

impl Default for FilePtr {
    fn default() -> Self {
        FilePtr {
            file: None,
            info: None,
        }
    }
}

impl PartialEq for FilePtr {
    fn eq(&self, other: &Self) -> bool {
        if self.file.is_none() && other.file.is_none() {
            true
        } else {
            false
        }
    }
}

impl CastFrom<Null> for FilePtr {
    fn cast_from(_: &mut Null) -> Self {
        FilePtr {
            file: None,
            info: None,
        }
    }
}

impl CastFrom<FilePtr> for FilePtr {
    fn cast_from(file: &mut FilePtr) -> Self {
        FilePtr {
            file: file.file,
            info: file.info,
        }
    }
}

macro_rules! eof {
    () => {
        -1
    };
}
pub(crate) use eof;

macro_rules! EOF {
    () => {
        -1
    };
}
pub(crate) use EOF;

pub fn fopen(mut filename: CStr, mut mode: CStr) -> FilePtr {
    if c_strcmp!(mode, cstr!("r")) == 0 || c_strcmp!(mode, cstr!("rb")) == 0 {
        let file = File::open(filename.to_string());
        match file {
            Ok(file) => {
                let boxed_file = Box::new(file);
                let leaked_file = unsafe { NonNull::new(Box::into_raw(boxed_file)).unwrap() };
                let boxed_info = Box::new(FileInfo {
                    byte_buf: 0,
                    buf_unread: false,
                });
                let leaked_info = unsafe { NonNull::new(Box::into_raw(boxed_info)).unwrap() };
                FilePtr {
                    file: Some(leaked_file),
                    info: Some(leaked_info),
                }
            }
            Err(_) => null!(),
        }
    } else if c_strcmp!(mode, cstr!("w")) == 0 || c_strcmp!(mode, cstr!("wb")) == 0 {
        let file = File::create(filename.to_string());
        match file {
            Ok(file) => {
                let boxed_file = Box::new(file);
                let leaked_file = unsafe { NonNull::new(Box::into_raw(boxed_file)).unwrap() };
                let boxed_info = Box::new(FileInfo {
                    byte_buf: 0,
                    buf_unread: false,
                });
                let leaked_info = unsafe { NonNull::new(Box::into_raw(boxed_info)).unwrap() };
                FilePtr {
                    file: Some(leaked_file),
                    info: Some(leaked_info),
                }
            }
            Err(_) => null!(),
        }
    } else {
        panic!("Invalid mode");
    }
}

pub fn fclose(mut file: &mut FilePtr) -> i32 {
    if *file != null!() {
        unsafe {
            Box::from_raw(file.file.unwrap().as_ptr());
            Box::from_raw(file.info.unwrap().as_ptr());
        }
        file.file = None;
        return 0;
    }
    return eof!();
}

pub fn fgetc(mut file: &mut FilePtr) -> i32 {
    if *file == null!() {
        return eof!();
    }
    unsafe {
        if file.info.as_mut().unwrap().as_mut().buf_unread {
            file.info.as_mut().unwrap().as_mut().buf_unread = false;
            return file.info.as_mut().unwrap().as_mut().byte_buf as i32;
        } else {
            let mut buf: [u8; 1] = [0; 1];
            match file.file.as_mut().unwrap().as_mut().read(&mut buf) {
                Ok(1) => buf[0] as i32,
                _ => eof!(),
            }
        }
    }
}

pub fn ungetc(mut c: i32, mut file: &mut FilePtr) -> i32 {
    if *file == null!() {
        return eof!();
    }
    unsafe {
        file.info.as_mut().unwrap().as_mut().byte_buf = c as u8;
        file.info.as_mut().unwrap().as_mut().buf_unread = true;
    }
    c
}

pub fn fread(mut ptr: Ptr<u8>, mut size: usize, mut count: usize, mut file: &mut FilePtr) -> usize {
    if *file == null!() {
        return 0;
    }
    let mut read_size = size * count;
    let mut buf_read = 0;
    let mut read_count = 0;
    unsafe {
        if file.info.as_mut().unwrap().as_mut().buf_unread {
            file.info.as_mut().unwrap().as_mut().buf_unread = false;
            ptr[0] = file.info.as_mut().unwrap().as_mut().byte_buf;
            read_size -= 1;
            buf_read = 1;
            read_count = 1;
        }
    }
    let mut buf: Vec<u8> = vec![0; read_size];
    match unsafe { file.file.as_mut().unwrap().as_mut().read(&mut buf) } {
        Ok(read) => {
            read_count += read / size;
            for i in 0..read {
                ptr[i + buf_read] = buf[i];
            }
            read_count
        }
        _ => 0,
    }
}

pub fn fwrite(
    mut ptr: Ptr<u8>,
    mut size: usize,
    mut count: usize,
    mut file: &mut FilePtr,
) -> usize {
    if *file == null!() {
        return 0;
    }
    let mut write_size = size * count;
    let mut buf: Vec<u8> = vec![0; write_size];
    for i in 0..write_size {
        buf[i] = ptr[i];
    }
    match unsafe { file.file.as_mut().unwrap().as_mut().write(&buf) } {
        Ok(write) => write / size,
        _ => 0,
    }
}

pub fn fgets(mut s: Ptr<u8>, mut n: i32, mut file: &mut FilePtr) -> Ptr<u8> {
    if *file == null!() {
        return null!();
    }
    let mut i = 0;
    let mut buf = vec![0; n as usize];
    while i < n - 1 {
        let c = fgetc(file);
        if c == eof!() {
            i = 0;
            break;
        }
        buf[i as usize] = c as u8;
        i += 1;
        if c == '\n' as i32 {
            break;
        }
    }
    if i == 0 {
        return null!();
    }
    for j in 0..i {
        s[j] = buf[j as usize];
    }
    s[i] = 0;
    s
}

pub fn fseek(mut file: &mut FilePtr, mut offset: i64, mut whence: i32) -> i32 {
    if *file == null!() {
        return -1;
    }
    let whence = match whence {
        seek_set!() => std::io::SeekFrom::Start(offset as u64),
        seek_cur!() => std::io::SeekFrom::Current(offset),
        seek_end!() => std::io::SeekFrom::End(offset),
        _ => panic!("Invalid whence"),
    };

    match unsafe { file.file.as_mut().unwrap().as_mut().seek(whence) } {
        Ok(pos) => 0,
        _ => -1,
    }
}

pub fn ftell(mut file: &mut FilePtr) -> i64 {
    if *file == null!() {
        return -1;
    }
    match unsafe {
        file.file
            .as_mut()
            .unwrap()
            .as_mut()
            .seek(std::io::SeekFrom::End(0))
    } {
        Ok(pos) => pos as i64,
        _ => -1,
    }
}

pub fn remove(mut filename: CStr) -> i32 {
    match std::fs::remove_file(filename.to_string()) {
        Ok(_) => 0,
        _ => -1,
    }
}

macro_rules! SEEK_SET {
    () => {
        0
    };
}
pub(crate) use SEEK_SET;

macro_rules! SEEK_CUR {
    () => {
        1
    };
}
pub(crate) use SEEK_CUR;

macro_rules! SEEK_END {
    () => {
        2
    };
}
pub(crate) use SEEK_END;

macro_rules! seek_set {
    () => {
        0
    };
}
pub(crate) use seek_set;

macro_rules! seek_cur {
    () => {
        1
    };
}
pub(crate) use seek_cur;

macro_rules! seek_end {
    () => {
        2
    };
}
pub(crate) use seek_end;

macro_rules! c_fseek {
    ($file:expr, $offset:expr, $whence:expr) => {
        fseek(&mut $file, $offset, $whence)
    };
}

pub(crate) use c_fseek;

macro_rules! c_ftell {
    ($file:expr) => {
        ftell(&mut $file)
    };
}

pub(crate) use c_ftell;

macro_rules! c_fopen {
    ($filename:expr, $mode:expr) => {
        fopen($filename, $mode)
    };
}
pub(crate) use c_fopen;

macro_rules! c_fclose {
    ($file:expr) => {
        fclose(&mut $file)
    };
}
pub(crate) use c_fclose;

macro_rules! c_fgetc {
    ($file:expr) => {
        fgetc(&mut $file).cast()
    };
}
pub(crate) use c_fgetc;

macro_rules! c_ungetc {
    ($c:expr, $file:expr) => {
        ungetc($c, &mut $file)
    };
}
pub(crate) use c_ungetc;

macro_rules! c_fread {
    ($ptr:expr, $size:expr, $count:expr, $file:expr) => {
        fread($ptr.cast(), $size.cast(), $count.cast(), &mut $file).cast()
    };
}
pub(crate) use c_fread;

macro_rules! c_fwrite {
    ($ptr:expr, $size:expr, $count:expr, $file:expr) => {
        fwrite($ptr.cast(), $size.cast(), $count.cast(), &mut $file).cast()
    };
}
pub(crate) use c_fwrite;

macro_rules! c_fgets {
    ($s:expr, $n:expr, $file:expr) => {
        fgets($s.cast(), $n, &mut $file).cast()
    };
}
pub(crate) use c_fgets;

macro_rules! c_remove {
    ($filename:expr) => {
        remove($filename.cast())
    };
}

pub(crate) use c_remove;
