use crate::check_type::LsFileType;
use crate::check_type::*;
use crate::command_settings::CommandSettings;
use crate::named_direntry_vec::NamedDirEntriesVec;
use crate::permissions::*;
use crate::symlink::*;
use crate::utils::*;
use crate::DirEntryData;
use chrono::{DateTime, Local};
use std::ffi::OsString;
use std::fs;
use std::fs::Metadata;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use users::{get_group_by_gid, get_user_by_uid};

use std::fs::DirEntry;
pub fn get_entries(path: &str, command_settings: &CommandSettings) -> Vec<NamedDirEntriesVec> {
    let mut direntries: Vec<NamedDirEntriesVec> = Vec::new();

    if command_settings.is_d_treat_currentdir {
        let mut entries: Vec<DirEntryData> = Vec::new();

        let current_dir_entry = gen_current_direntrydata(command_settings);
        entries.push(current_dir_entry);
        let d = NamedDirEntriesVec {
            name: ".".to_string(),
            entries: entries,
        };
        direntries.push(d);
    } else {
        add_entries(&mut direntries, path, &command_settings);

        //add . and .. if is_all
        if command_settings.is_a_all_including_current_parent {
            add_current_and_parent(&mut direntries, command_settings);
        }
    }
    return direntries;
}

fn add_current_and_parent(
    entries: &mut Vec<NamedDirEntriesVec>,
    command_settings: &CommandSettings,
) {
    let current_dir_entry = gen_current_direntrydata(command_settings);
    let parent_dir_entry = gen_parent_direntrydata(command_settings);

    entries[0].entries.insert(0, parent_dir_entry);
    entries[0].entries.insert(0, current_dir_entry);
}

pub fn get_direntrydata_by_path(path: String, command_settings: &CommandSettings) -> DirEntryData {
    let path = PathBuf::from(path);
    let mut name = get_entry_name(&path, None, None, command_settings);

    let is_symlink = is_symlink(&path);
    let is_symlink2 = is_symlink2(&path);

    ///GET METADATA
    let metadata = if command_settings.is_F_show_filetype_do_not_follow_symbolic_links {
        fs::symlink_metadata(&path) // Do not follow symbolic links
    } else if command_settings.is_L_evaluate_all_symlink_fileinfo_for_target {
        //println!("ISL");
        fs::metadata(&path)
        // fs::metadata(&path) // Follow symbolic links
    } else {
        if is_symlink {
            fs::symlink_metadata(&path) // Do not follow symbolic links
        } else {
            fs::metadata(&path) // Follow symbolic links
        }
    };

    let target_metadata = fs::metadata(&path);

    if let Ok(metadata) = metadata {
        if let Ok(target_metadata) = target_metadata {
            name = get_entry_name(
                &path,
                Some(&metadata),
                Some(&target_metadata),
                command_settings,
            );
            return gen_entrydata_from_metadata(path, command_settings, metadata, target_metadata);
        } else {
            return get_entrydata_fallback_after_failed_metadata(path, command_settings);
        }
    } else {
        return get_entrydata_fallback_after_failed_metadata(path, command_settings);
    }
}

pub fn get_entrydata_fallback_after_failed_metadata(
    path: PathBuf,
    command_settings: &CommandSettings,
) -> DirEntryData {
    let mut name = get_entry_name(&path, None, None, command_settings);

    //   println!("Could not read metadata for {}", path.display());
    return DirEntryData {
        permissions: None,
        nlinks: None,
        gid: None,
        created_time: None,
        symlink_target_name: None,
        size_in_blocks: None,
        inode: None,
        user_name: None,
        modified_time: None,
        inode_and_name: None,
        blocks_and_name: None,
        group_name: None,
        has_extended_attributes: None,
        blocks: None,
        file_type: None,
        uid: None,
        modified_time_str: None,
        name: name,
        path: path.display().to_string(),
        is_dir: false,
        is_symlink: None,
        size: 0,
    };
}
pub fn get_entry_name(
    path: &PathBuf,
    metadata: Option<&Metadata>,
    target_metadata: Option<&Metadata>,
    command_settings: &CommandSettings,
) -> String {
    let mut name = "".to_string();
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            name = file_name_str.to_string()
        }
    }
    if command_settings.is_q_force_nonprintable_as_questionmarks {
        let os_string = OsString::from(name);
        name = sanitize_filename(os_string);
    }
    if command_settings.is_F_show_filetype_do_not_follow_symbolic_links {
        if let Some(metadata) = metadata {
            if let Some(target_metadata) = target_metadata {
                //            let type_=get_entry_ty
                let mut symbol =
                    get_entry_symbol_final(&path, &metadata, &target_metadata, command_settings);

                //let mut symbol=get_entry_symbol2_final(path,type_,metadata,target_metadata);
                name = format!("{}{}", name, symbol)
            }
        }
    }

    if (command_settings.is_l_long
        || command_settings.is_g_hide_user
        || command_settings.is_o_hide_group)
        && !command_settings.is_L_evaluate_all_symlink_fileinfo_for_target
    {
        let is_symlink = is_symlink(&path);

        if is_symlink {
            let mut symlink_target_name = get_symlink_target_name(&path);
            name = format!("{} -> {}", name, symlink_target_name)
        }
    }

    return name;
}
pub fn gen_entrydata_from_metadata(
    path: PathBuf,
    command_settings: &CommandSettings,
    metadata: Metadata,
    target_metadata: Metadata,
) -> DirEntryData {
    let type_ = get_file_type(&path, &metadata);
    let target_type = get_symlink_target_file_type(&path);

    let mut name = get_entry_name(
        &path,
        Some(&metadata),
        Some(&target_metadata),
        command_settings,
    );

    let mut symlink_target_name = get_symlink_target_name(&path);
    let is_symlink = is_symlink(&path);
    let is_symlink2 = is_symlink2(&path);

    let size = get_entry_size(&path, &metadata).unwrap();

    //date
    let modified_time = metadata.modified().unwrap();
    let modified_time = modified_time.duration_since(std::time::UNIX_EPOCH).unwrap();

    //FILE TYPE LETTER
    let mut final_file_type_letter_symbol =
        get_entry_type_letter_final(&path, &metadata, &target_metadata, command_settings);
    //SYMBOL
    let mut final_file_type_letter_symbol2 =
        get_entry_symbol_final(&path, &metadata, &target_metadata, command_settings);

    let mut permissions = metadata.permissions();
    let mut permissions_str = format_mode(permissions.mode());

    let nlinks = metadata.nlink();
    let uid = metadata.uid();
    let gid = metadata.gid();

    let user_name = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned());
    let group_name = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned());

    // let size = metadata.size();

    let modified = DateTime::<Local>::from(metadata.modified().unwrap());
    let mut formatted_time = modified.format("%b %d %H:%M").to_string();
    if is_symlink && !command_settings.is_L_evaluate_all_symlink_fileinfo_for_target {
        formatted_time = get_symlink_modified(&path).unwrap();
        permissions_str = get_symlink_permissions(&path).unwrap();
    }

    let size_in_blocks = (metadata.len() as f64 / 1024.0).ceil() as u64;
    let has_extended_attributes = has_extended_attributes(&path);
    let mut blocks = metadata.blocks();
    if is_symlink {
        if !command_settings.is_L_evaluate_all_symlink_fileinfo_for_target {
            blocks = get_symlink_blocks(&path).unwrap();
        }
    }

    let mut inode = metadata.ino();
    if is_symlink {
        inode = get_symlink_inode(&path).unwrap();
    }
    let inode_and_name = format!("{:<8} {}", inode, name.clone().to_string());
    let blocks_and_name = format!("{:<8} {}", blocks, name.clone().to_string());

    let mut is_dir = false;
    if command_settings.is_p_add_slash {
        if matches!(type_, LsFileType::dir) {
            is_dir = true;
            name = format!("{}/", name);
        }
    }
    return DirEntryData {
        file_type: Some(final_file_type_letter_symbol),
        name: name,
        inode_and_name: Some(inode_and_name),
        blocks_and_name: Some(blocks_and_name),
        modified_time: Some(metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)),
        created_time: Some(metadata.created().unwrap_or(SystemTime::UNIX_EPOCH)),
        has_extended_attributes: Some(has_extended_attributes),
        uid: Some(uid),
        blocks: Some(blocks),
        modified_time_str: Some(formatted_time),
        gid: Some(gid),
        inode: Some(inode),
        size_in_blocks: Some(size_in_blocks),
        user_name: user_name,
        symlink_target_name: Some(symlink_target_name),
        group_name: group_name,
        nlinks: Some(nlinks),
        permissions: Some(permissions_str),
        path: path.display().to_string(),
        is_dir: is_dir,
        is_symlink: Some(is_symlink),
        size: size,
    };
}

pub fn get_entry_type_letter_final(
    path: &PathBuf,
    metadata: &Metadata,
    target_metadata: &Metadata,
    command_settings: &CommandSettings,
) -> String {
    let type_ = get_file_type(&path, &metadata);
    let target_type = get_symlink_target_file_type(&path);
    let is_symlink = is_symlink(&path);

    //FILE TYPE
    let mut file_type_letter = get_entry_letter_symbol_by_type(&path, &type_);
    let mut target_file_type_letter = get_entry_letter_symbol_by_type(&path, &target_type);
    let mut final_type_letter = file_type_letter;
    if is_symlink {
        if command_settings.is_L_evaluate_all_symlink_fileinfo_for_target {
            final_type_letter = target_file_type_letter;
        }
    }
    return final_type_letter;
}

pub fn get_entry_symbol_final(
    path: &PathBuf,
    metadata: &Metadata,
    target_metadata: &Metadata,
    command_settings: &CommandSettings,
) -> String {
    let type_ = get_file_type(&path, &metadata);
    let target_type = get_symlink_target_file_type(&path);
    let is_symlink = is_symlink(&path);

    //FILE TYPE
    let mut file_type_letter_symbol = get_entry_symbol_by_type(&path, &type_);
    let mut target_file_type_letter_symbol = get_entry_symbol_by_type(&path, &target_type);
    let mut final_file_type_letter_symbol = file_type_letter_symbol;
    if is_symlink {
        if command_settings.is_L_evaluate_all_symlink_fileinfo_for_target {
            final_file_type_letter_symbol = target_file_type_letter_symbol;
        }
    }
    return final_file_type_letter_symbol;
}
pub fn get_entry_symbol(path: &PathBuf, metadata: &Metadata) -> String {
    let type_ = get_file_type(&path, &metadata);

    match type_ {
        LsFileType::symlink => "@".to_string(),
        LsFileType::dir => "/".to_string(),
        LsFileType::fifo => "|".to_string(),
        LsFileType::exe => "*".to_string(),
        _ => "/".to_string(),
    }
}

pub fn get_entry_symbol_by_type(path: &PathBuf, type_: &LsFileType) -> String {
    match type_ {
        LsFileType::symlink => "@".to_string(),
        LsFileType::dir => "/".to_string(),
        LsFileType::fifo => "|".to_string(),
        LsFileType::exe => "*".to_string(),
        _ => "".to_string(),
    }
}
pub fn get_symlink_target_file_type(symlink_path: &Path) -> LsFileType {
    let metadata = fs::metadata(symlink_path);
    if let Ok(metadata) = metadata {
        // Follows the symlink
        let file_type = metadata.file_type();

        if file_type.is_dir() {
            LsFileType::dir
        } else if file_type.is_file() {
            LsFileType::file
        } else {
            LsFileType::unknown
        }
    } else {
        LsFileType::unknown
    }
}
pub fn get_entry_letter_symbol(
    path: &PathBuf,
    metadata: &Metadata,
    command_settings: &CommandSettings,
) -> String {
    let type_ = get_file_type(path, metadata);

    match type_ {
        LsFileType::symlink => "l".to_string(),
        LsFileType::dir => "d".to_string(),
        LsFileType::fifo => "?".to_string(),
        LsFileType::exe => "?".to_string(),
        LsFileType::unknown => "UNKNOWN".to_string(),

        _ => "-".to_string(),
    }
}

pub fn get_entry_letter_symbol_by_type(path: &PathBuf, type_: &LsFileType) -> String {
    match type_ {
        LsFileType::symlink => "l".to_string(),
        LsFileType::dir => "d".to_string(),
        LsFileType::fifo => "f".to_string(),
        LsFileType::exe => "e".to_string(),
        LsFileType::unknown => "UNKNOWN".to_string(),

        _ => "-".to_string(),
    }
}

pub fn add_entries(
    entries_vec: &mut Vec<NamedDirEntriesVec>,
    path: &str,
    command_settings: &CommandSettings,
) {
    let mut direntries_data_vec: Vec<DirEntryData> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        let collected: Vec<_> = entries.filter_map(Result::ok).collect();
        for entry in collected {
            if should_display(&entry, command_settings) {
                let p = entry.path();
                let name = entry.file_name();
                let pstr = p.to_str().unwrap();
                // println!("Add {}",pstr);

                let data = get_direntrydata(entry, command_settings);
                direntries_data_vec.push(data);
                if command_settings.is_R_recursive && p.is_dir() {
                    // Avoiding infinite loop by not re-listing '.' or '..'
                    if name != "." && name != ".." {
                        let mut dirs: Vec<NamedDirEntriesVec> = Vec::new();
                        add_entries(&mut dirs, pstr, command_settings);
                        for dir in dirs {
                            entries_vec.push(dir);
                        }
                    }
                }
            }
        }
        let d: NamedDirEntriesVec = NamedDirEntriesVec {
            name: path.to_string(),
            entries: direntries_data_vec,
        };

        entries_vec.push(d);
    } else {
        //   eprintln!("Failed to read directory: {}", path);
    }
}

fn gen_current_direntrydata(command_settings: &CommandSettings) -> DirEntryData {
    let mut d = get_direntrydata_by_path(".".to_string(), &command_settings);
    d.name = ".".to_string();
    return d;
}
fn gen_parent_direntrydata(command_settings: &CommandSettings) -> DirEntryData {
    let mut d = get_direntrydata_by_path("..".to_string(), &command_settings);
    d.name = "..".to_string();
    return d;
}

fn should_display(entry: &DirEntry, commandsettings: &CommandSettings) -> bool {
    //    let entryname=entry.file_name();
    //  let name=entryname.to_str().unwrap();
    if commandsettings.is_a_all_including_current_parent {
        //  let show=true;
        // println!("Check {} {}",name,show);
        return true;
    } else if commandsettings.is_A_all_excluding_current_parent {
        return true;
        let show = !entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.starts_with('.'));
        // println!("Check {} {}",name,show);
        return show;
    }
    return !entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'));
}

fn get_direntrydata(entry: DirEntry, command_settings: &CommandSettings) -> DirEntryData {
    let mut name = entry.file_name().to_str().unwrap().to_string();
    let path = entry.path().display().to_string();
    return get_direntrydata_by_path(path, command_settings);
}

fn has_extended_attributes<P: AsRef<Path>>(path: P) -> bool {
    match xattr::list(path.as_ref()) {
        Ok(mut attrs) => attrs.next().is_some(),
        Err(_) => false,
    }
}
