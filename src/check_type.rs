use std::fs::{File, Metadata};
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use std::path::PathBuf;
use std::os::unix::fs::FileTypeExt;
pub enum LsFileType{
    dir,
    exe,
    symlink,
    fifo,
    file, 
    unknown
}

pub fn get_file_type(path:&PathBuf,metadata:&Metadata) -> LsFileType{
    if is_entry_directory(&path,&metadata) {
        return LsFileType::dir;
    }else if is_symlink(&path){
        return LsFileType::symlink;
    }else if is_entry_executable(&path,&metadata){
        return LsFileType::exe;
    }else if is_entry_fifo(&path,&metadata){
        return LsFileType::fifo;
    }else {
        return LsFileType::file;
    }

}
pub fn is_entry_directory<P: AsRef<Path>>(path: P,metadata:&Metadata) -> bool {
    metadata.is_dir()
}

pub fn is_symlink2(path: &Path) -> bool {
  if let Ok(   metadata) = fs::symlink_metadata(path){
    return true
  }else{
    return false;
  }
}
pub fn is_symlink(path: &Path) -> bool {
    let metadata = fs::symlink_metadata(path).unwrap();
    return metadata.file_type().is_symlink();
}
pub fn is_entry_executable<P: AsRef<Path>>(path: &P,metadata:&Metadata) -> bool {
    let permissions = metadata.permissions();
    (permissions.mode() & 0o111) != 0 // Check if any execute bits are set (owner, group, or others)
}

pub fn is_entry_fifo<P: AsRef<Path>>(path: P,metadata:&Metadata) -> bool {
    metadata.file_type().is_fifo()
}