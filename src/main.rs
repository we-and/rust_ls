use clap::{App, Arg};

use std::fs;
use std::fs::DirEntry;
use std::io::Write;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use std::time::Duration;
extern crate xattr;
use std::io;

use std::fs::File;

use xattr::{FileExt, XAttrs};
use users::{get_user_by_uid, get_group_by_gid};
extern crate atty;
extern crate term_size;
use term_size::dimensions;

use atty::{is, Stream};

struct DirEntryData {
    name: String,
    path: String,
    is_dir: bool,
    is_symlink: Option<bool>,

    size: u64,
    modified_time: Duration,
    created_time:Option<SystemTime>,
    modified_time_str: Option<String>,
    symlink_target_name: Option<String>,

    permissions: Option<String>,
    nlinks: Option<u64>,
    uid: Option<u32>,
    gid: Option<u32>,
    user_name:Option<String>,
    blocks:Option<u64>,
    has_extended_attributes:Option<bool>,
    group_name:Option<String>,
    file_type: Option<String>,
}

struct CommandSettings {

    is_sorted_by_status_change_time:bool,
    is_sorted_by_size:bool,
    is_all: bool,
    is_d: bool,
    is_all_excluding_dot: bool,
    is_long: bool,
    is_recursive: bool,
    do_not_follow_symbolic_links: bool,
}
struct NamedDirEntriesVec {
    name: String,
    entries: Vec<DirEntryData>,
}

fn gen_current_direntrydata(command_settings: &CommandSettings) -> DirEntryData {
    let mut d = get_data_by_path(".".to_string(), &command_settings);
    d.name = ".".to_string();
    return d;
}
fn gen_parent_direntrydata(command_settings: &CommandSettings) -> DirEntryData {
    let mut d = get_data_by_path("..".to_string(), &command_settings);
    d.name = "..".to_string();
    return d;
}

fn main() {
    let matches = App::new("Rust ar")
        .version("0.1.0")
        .author("J Dumont")
        .about("Implements an ar command in Rust")
        .arg(Arg::new("path")
            .default_value(".")
            .help("The path to list"))    
        .arg(Arg::new("A")
             .short('A')
             .takes_value(false)
             .help("Write out all directory entries, including those whose names begin with a <period> ( '.' ) but excluding the entries dot and dot-dot (if they exist)."))
        .arg(Arg::new("C")
             .short('C')
             .takes_value(false)
             .help("Write multi-text-column output with entries sorted down the columns, according to the collating sequence. The number of text columns and the column separator characters are unspecified, but should be adapted to the nature of the output device. This option disables long format output."))
        .arg(Arg::new("F")
             .short('F')
             .takes_value(false)
             .help("Do not follow symbolic links named as operands unless the -H or -L options are specified. Write a <slash> ( '/' ) immediately after each pathname that is a directory, an <asterisk> ( '*' ) after each that is executable, a <vertical-line> ( '|' ) after each that is a FIFO, and an at-sign ( '@' ) after each that is a symbolic link. For other file types, other symbols may be written."))
        .arg(Arg::new("H")
             .short('H')
             .takes_value(false)
             .help("Evaluate the file information and file type for symbolic links specified on the command line to be those of the file referenced by the link, and not the link itself; however, ls shall write the name of the link itself and not the file referenced by the link."))
        .arg(Arg::new("L")
             .short('L')
             .takes_value(false)
             .help("Evaluate the file information and file type for all symbolic links (whether named on the command line or encountered in a file hierarchy) to be those of the file referenced by the link, and not the link itself; however, ls shall write the name of the link itself and not the file referenced by the link. When -L is used with -l, write the contents of symbolic links in the long format (see the STDOUT section)."))
        .arg(Arg::new("R")
             .short('R')
             .takes_value(false)
             .help("Recursively list subdirectories encountered. When a symbolic link to a directory is encountered, the directory shall not be recursively listed unless the -L option is specified. The use of -R with -d or -f produces unspecified results."))
        .arg(Arg::new("S")
             .short('S')
             .takes_value(false)
             .help("Sort with the primary key being file size (in decreasing order) and the secondary key being filename in the collating sequence (in increasing order)."))
        .arg(Arg::new("a")
             .short('a')
             .takes_value(false)
             .help("Write out all directory entries, including those whose names begin with a <period> ( '.' )."))
        .arg(Arg::new("c")
             .short('c')
             .takes_value(false)
             .help("Use time of last modification of the file status information (see XBD <sys/stat.h>) instead of last modification of the file itself for sorting ( -t) or writing (-l)."))
        .arg(Arg::new("d")
             .short('d')
             .takes_value(false)
             .help("Do not follow symbolic links named as operands unless the -H or -L options are specified. Do not treat directories differently than other types of files. The use of -d with -R or -f produces unspecified results."))
        .arg(Arg::new("f")
             .short('f')
             .takes_value(false)
             .help("List the entries in directory operands in the order they appear in the directory. The behavior for non-directory operands is unspecified. This option shall turn on -a. When -f is specified, any occurrences of the -r, -S, and -t options shall be ignored and any occurrences of the -A, [XSI] [Option Start] -g, [Option End] -l, -n, [XSI] [Option Start] -o, [Option End] and -s options may be ignored. The use of -f with -R or -d produces unspecified results."))
        .arg(Arg::new("g")
             .short('g')
             .takes_value(false)
             .help("Turn on the -l (ell) option, but disable writing the file's owner name or number. Disable the -C, -m, and -x options."))
        .arg(Arg::new("i")
             .short('i')
             .takes_value(false)
             .help("For each file, write the file's file serial number (see stat() in the System Interfaces volume of POSIX.1-2017)."))
        .arg(Arg::new("k")
             .short('k')
             .takes_value(false)
             .help("Set the block size for the -s option and the per-directory block count written for the -l, -n, -s, [XSI] [Option Start] -g, and -o [Option End] options (see the STDOUT section) to 1024 bytes."))
        .arg(Arg::new("l")
             .short('l')
             .takes_value(false)
             .help("Do not follow symbolic links named as operands unless the -H or -L options are specified. Write out in long format (see the STDOUT section). Disable the -C, -m, and -x options."))
        .arg(Arg::new("m")
             .short('m')
             .takes_value(false)
             .help("Stream output format; list pathnames across the page, separated by a <comma> character followed by a <space> character. Use a <newline> character as the list terminator and after the separator sequence when there is not room on a line for the next list entry. This option disables long format output."))
        .arg(Arg::new("n")
             .short('n')
             .takes_value(false)
             .help("Turn on the -l (ell) option, but when writing the file's owner or group, write the file's numeric UID or GID rather than the user or group name, respectively. Disable the -C, -m, and -x options."))
        .arg(Arg::new("o")
             .short('o')
             .takes_value(false)
             .help("Turn on the -l (ell) option, but disable writing the file's group name or number. Disable the -C, -m, and -x options."))
        .arg(Arg::new("p")
             .short('p')
             .takes_value(false)
             .help("Write a <slash> ( '/' ) after each filename if that file is a directory."))
             .arg(Arg::new("q")
             .short('q')
             .takes_value(false)
             .help("Force each instance of non-printable filename characters and <tab> characters to be written as the <question-mark> ( '?' ) character. Implementations may provide this option by default if the output is to a terminal device."))
             .arg(Arg::new("r")
             .short('r')
             .takes_value(false)
             .help("Reverse the order of the sort to get reverse collating sequence oldest first, or smallest file size first depending on the other options given."))
             .arg(Arg::new("s")
             .short('s')
             .takes_value(false)
             .help("Indicate the total number of file system blocks consumed by each file displayed. If the -k option is also specified, the block size shall be 1024 bytes; otherwise, the block size is implementation-defined."))
             .arg(Arg::new("t")
             .short('t')
             .takes_value(false)
             .help("Sort with the primary key being time modified (most recently modified first) and the secondary key being filename in the collating sequence. For a symbolic link, the time used as the sort key is that of the symbolic link itself, unless ls is evaluating its file information to be that of the file referenced by the link (see the -H and -L options)."))
             .arg(Arg::new("u")
             .short('u')
             .takes_value(false)
             .help("Use time of last access (see XBD <sys/stat.h>) instead of last modification of the file for sorting (-t) or writing (-l).
             "))
             .arg(Arg::new("x")
             .short('x')
             .takes_value(false)
             .help("The same as -C, except that the multi-text-column output is produced with entries sorted across, rather than down, the columns. This option disables long format output.
             "))
             .arg(Arg::new("1")
             .short('1')
             .takes_value(false)
             .help("Force output to be one entry per line. This option does not disable long format output. (Long format output is enabled by [XSI] [Option Start] -g, [Option End] -l (ell), -n, and [XSI] [Option Start] -o; [Option End] and disabled by -C, -m, and -x.)
            "))
        .get_matches();
 
    //FLAGS
    let path = matches.value_of("path").unwrap();
    let is_all = matches.is_present("a");
    let is_all_excluding_dot = matches.is_present("A");
    let is_long = matches.is_present("l");
    let is_recursive = matches.is_present("R");
    let do_not_follow_symbolic_links = matches.is_present("F");
    let is_d=matches.is_present("d");
    let is_sorting=matches.is_present("S");
    let is_sorted_by_status_change_time=matches.is_present("c");
    //COMMAND_SETTINGS
    let command_settings = CommandSettings {
        is_all_excluding_dot: is_all_excluding_dot,
        is_long: is_long,
        is_sorted_by_status_change_time:is_sorted_by_status_change_time,
        is_recursive: is_recursive,
        is_sorted_by_size:is_sorting,
        is_d:is_d,
        is_all: is_all,
        do_not_follow_symbolic_links: do_not_follow_symbolic_links,
    };

    list_directory(path, &command_settings);
}

fn get_entries(path: &str, command_settings: &CommandSettings) -> Vec<NamedDirEntriesVec> {
    let mut direntries: Vec<NamedDirEntriesVec> = Vec::new();


    if command_settings.is_d{
        let mut entries: Vec<DirEntryData> = Vec::new();

        let current_dir_entry = gen_current_direntrydata(command_settings);
        entries.push(current_dir_entry);
        let d=NamedDirEntriesVec{
            name:".".to_string(),
            entries:entries,
        };
        direntries.push(d);
    }else{

        add_entries(&mut direntries, path, &command_settings);

        //add . and .. if is_all
        if command_settings.is_all {
            add_current_and_parent(&mut direntries, command_settings);
        }
    }
    return direntries;
}

fn add_current_and_parent(entries: &mut Vec<NamedDirEntriesVec>, command_settings: &CommandSettings) {
    let current_dir_entry = gen_current_direntrydata(command_settings);
    let parent_dir_entry = gen_parent_direntrydata(command_settings);

    entries[0].entries.push(current_dir_entry);
    entries[0].entries.push(parent_dir_entry);
}

fn is_directory<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(e) => {
            eprintln!("Failed to read metadata: {}", e);
            false
        }
    }
}
fn format_mode(mode: u32) -> String {
    let user = mode_permissions((mode >> 6) & 7);
    let group = mode_permissions((mode >> 3) & 7);
    let other = mode_permissions(mode & 7);
    format!("{}{}{}", user, group, other)
}

fn mode_permissions(perm: u32) -> String {
    let mut perms = String::new();
    perms.push(if perm & 4 != 0 { 'r' } else { '-' });
    perms.push(if perm & 2 != 0 { 'w' } else { '-' });
    perms.push(if perm & 1 != 0 { 'x' } else { '-' });
    perms
}


fn has_extended_attributes<P: AsRef<Path>>(path: P) -> bool {
    match xattr::list(path.as_ref()) {
        Ok(mut attrs) => attrs.next().is_some(),
        Err(_) => false,
    }
}
fn find_symlink_target(path: &Path) -> io::Result<Option<PathBuf>> {
    // Check if the path is a symlink
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        // Read the symlink target
        fs::read_link(path).map(Some)
    } else {
        // Path is not a symlink
        Ok(None)
    }
}
fn get_symlink_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::symlink_metadata(path)?;
    Ok(metadata.len())  // `len()` returns the size of the symlink
}
fn get_data_by_path(path: String, command_settings: &CommandSettings) -> DirEntryData {
    let path = PathBuf::from(path);

    let mut name = "".to_string();
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            name = file_name_str.to_string()
        }
    }

    let is_dir = is_directory(&path);
    let is_exe = is_executable(&path);
    let is_fifo = is_fifo(&path);
    let metadata = if command_settings.do_not_follow_symbolic_links {
        fs::symlink_metadata(&path) // Do not follow symbolic links
    } else {
        fs::metadata(&path) // Follow symbolic links
    };

    let mut symlink_target_name="".to_string(); 
    let is_symlink=is_symlink(&path);

    let mut size=0;
    if is_symlink {
        size=get_symlink_size(&path).unwrap();
        
        match find_symlink_target(&path) {
            Ok(Some(target)) => { symlink_target_name= target.display().to_string();  },
            Ok(None) => println!("Not a symlink"),
            Err(e) => println!("Error: {}", e),
        }
    }

    if let Ok(metadata) = metadata {
        if !is_symlink{
        size = metadata.len();
        }
        let modified_time = metadata.modified().unwrap();
        let modified_time = modified_time.duration_since(std::time::UNIX_EPOCH).unwrap();

        if command_settings.do_not_follow_symbolic_links {
            if is_symlink {
                println!("SYM {}",is_symlink );
                name = format!("{}@", name)
            } else if is_dir {
                name = format!("{}/", name)
            } else if is_exe {
                name = format!("{}*", name)
            } else if is_fifo {
                name = format!("{}|", name)
            }
        }
        if command_settings.is_long{
          

            if is_symlink {
                      name = format!("{} -> {}", name,symlink_target_name)
            } 
        }

        let file_type = if metadata.file_type().is_dir() {
            "d"
        } else if metadata.file_type().is_file() {
            "-"
        } else if metadata.file_type().is_symlink() {
            "l"
        } else {
            "?"
        };
        let permissions = metadata.permissions();

        let permissions = format_mode(permissions.mode());
        let nlinks = metadata.nlink();
        let uid = metadata.uid();
        let gid = metadata.gid();

        let user_name = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned());
        let group_name = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned());

        // let size = metadata.size();
        let modified = DateTime::<Local>::from(metadata.modified().unwrap());
        let formatted_time = modified.format("%b %d %H:%M").to_string();

        
        let has_extended_attributes=has_extended_attributes(&path);
        let blocks=metadata.blocks();
        return DirEntryData {
            file_type: Some(file_type.to_string()),
            name: name,
created_time:Some( metadata.created().unwrap_or(SystemTime::UNIX_EPOCH)),
            has_extended_attributes:Some(has_extended_attributes),
            uid: Some(uid),
            blocks:Some(blocks),
            modified_time_str:Some(formatted_time),
            gid: Some(gid),
            user_name:user_name,
            symlink_target_name:Some(symlink_target_name),
            group_name:group_name,
            nlinks: Some(nlinks),
            permissions: Some(permissions),
            path: path.display().to_string(),
            is_dir: is_dir,
            is_symlink: Some(is_symlink),
            modified_time,
            size: size,
        };
    } else {
        println!("Could not read metadata for {}", path.display());
        return DirEntryData {
            permissions: None,
            nlinks: None,
            gid: None,
            created_time:None,
            symlink_target_name:None,
            user_name:None,
            group_name:None,
            has_extended_attributes:None,
            blocks:None,
            file_type: None,
            uid: None,
            modified_time_str:None,
            name: name,
            path: path.display().to_string(),
            is_dir: is_dir,
            is_symlink: None,
            modified_time: Duration::new(0, 0),
            size: 0,
        };
    }
}
fn get_direntrydata(entry: DirEntry, command_settings: &CommandSettings) -> DirEntryData {
    let mut name = entry.file_name().to_str().unwrap().to_string();
    let path = entry.path().display().to_string();
    return get_data_by_path(path, command_settings);
  
}
fn is_fifo<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.file_type().is_fifo(),
        Err(e) => {
            eprintln!("Failed to read metadata: {}", e);
            false
        }
    }
}
fn is_symlink(path: &Path) -> bool {
    let metadata = fs::symlink_metadata(path).unwrap();
    return metadata.file_type().is_symlink();
}
fn is_executable<P: AsRef<Path>>(path: &P) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Failed to read metadata: {}", e);
            return false;
        }
    };

    let permissions = metadata.permissions();
    (permissions.mode() & 0o111) != 0 // Check if any execute bits are set (owner, group, or others)
}

fn add_entries(entries_vec: &mut Vec<NamedDirEntriesVec>, path: &str, command_settings: &CommandSettings) {
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
                if command_settings.is_recursive && p.is_dir() {
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
        eprintln!("Failed to read directory: {}", path);
    }
}

fn list_directory(path: &str, command_settings: &CommandSettings) {
    let mut dirs: Vec<NamedDirEntriesVec> = get_entries(path, command_settings);
    
    sort_entries(&mut dirs,command_settings);

    if !command_settings.is_recursive {
        // Access the only element immutably
        let de = &dirs[0];
        display_entries(&de.entries, command_settings);
    } else {
        // Access all elements immutably
        for dir in dirs {
            if dir.name != "." {
                println!("\n{}:", dir.name);
            }
            display_entries(&dir.entries, command_settings);
        }
    }
}
fn sort_entries(dirs : &mut Vec<NamedDirEntriesVec>,commandsettings: &CommandSettings ){
   if commandsettings.is_sorted_by_status_change_time{

  // Sort entries alphabetically and case-insensitively within each directory list
  dirs.sort_by_key(|dir| dir.name.to_lowercase());

  // Sort entries alphabetically and case-insensitively within each directory list
  for dir in  dirs {
      //dir.entries.sort_by_key(|entry| entry.name.to_lowercase());
  
      dir.entries.sort_unstable_by_key(|entry| {
        entry.created_time
    });
    dir.entries.reverse(); 
  }


   }else       if !commandsettings.is_sorted_by_size{  //sort by name
        // Sort entries alphabetically and case-insensitively within each directory list
        dirs.sort_by_key(|dir| dir.name.to_lowercase());

        // Sort entries alphabetically and case-insensitively within each directory list
        for dir in  dirs {
            dir.entries.sort_by_key(|entry| entry.name.to_lowercase());
        }
    
    }else{ //sort by size
         // Sort entries alphabetically and case-insensitively within each directory list
         dirs.sort_by_key(|dir| dir.name.to_lowercase());


        for dir in  dirs {
            dir.entries.sort_unstable_by(|a, b| {
            let a_size = a.size;
            let b_size = b.size;
    
            // Primary sort by size (reverse order)
            b_size.cmp(&a_size)
                .then_with(|| {
                    // Secondary sort by filename (normal order)
                    a.name.cmp(&b.name)
                })
        });
        }
    }
}
fn should_display(entry: &DirEntry, commandsettings: &CommandSettings) -> bool {
    //    let entryname=entry.file_name();
    //  let name=entryname.to_str().unwrap();
    if commandsettings.is_all {
        //  let show=true;
        // println!("Check {} {}",name,show);
        return true;
    } else if commandsettings.is_all_excluding_dot {
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

fn display_entries(entries: &[DirEntryData], commandsettings: &CommandSettings) {
    if commandsettings.is_long {
     if !commandsettings.is_d
     {   let mut total:u64=0;
        for e in entries {
            let b = e.blocks.unwrap();
            total = total + b;
        
        }
        println!("total {}",total);
    }
        let max_user_length = entries.iter().map(|e| e.user_name.as_ref().unwrap().len()).max().unwrap_or(0);
        let max_group_length = entries.iter().map(|e| e.group_name.as_ref().unwrap().len()).max().unwrap_or(0);

        

        for e in entries {

            //EXTRA_ATTRIBUTES
            let mut extr_attr=" ";
            let has_attr=e.has_extended_attributes.unwrap();
            if(has_attr){
                extr_attr="@";
            }

            let mut name=e.name.to_string();

            println!(
                "{}{}{} {} {:width$}  {:width2$}{:>7} {} {}",
                e.file_type.as_ref().unwrap(),        e.permissions.as_ref().unwrap(),extr_attr , e.nlinks.unwrap(), e.user_name.as_ref().unwrap(),e. group_name.as_ref().unwrap(), e.size, e.modified_time_str.as_ref().unwrap(), name, width = max_user_length,
                width2 = max_group_length
            );
           
        }
    }else{
    if atty::is(Stream::Stdout) {
        if let Some((width, _)) = dimensions() {
            let mut max_len = 0;
            for entry in entries {
                let len = entry.name.len();
                if len > max_len {
                    max_len = len;
                }
            }
            let columns = width / (max_len + 8); // +8 for padding and tab space
            let rows = (entries.len() + columns - 1) / columns; // Calculate required rows

            for row in 0..rows {
                for col in 0..columns {
                    if let Some(entry) = entries.get(col * rows + row) {
                        // Calculate correct index for column-first ordering
                        print!("{:<width$}\t", entry.name, width = max_len);
                    }
                }
                println!(); // End the line after each row
            }
        } else {
            // Fallback if terminal dimensions can't be fetched
            for entry in entries {
                println!("{}", entry.name);
            }
        }
    } else {
        // Non-TTY output
        for entry in entries {
            println!("{}", entry.name);
        }
    }
}
}
