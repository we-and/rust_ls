use clap::{App, Arg};

use std::path::Path;
use std::time::Duration;

use std::fs::{self, DirEntry};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::FileTypeExt; 
extern crate atty;
extern crate term_size;
use term_size::dimensions;

use atty::{is, Stream};

struct DirEntryData{
    name:String,
    path:String,
    is_dir:bool,
    is_symlink:bool,
    size:u64,
    modified_data:Duration,
}

struct CommandSettings {
    is_all:bool,
    is_all_excluding_dot:bool,
    is_long:bool,
    is_recursive:bool,
    do_not_follow_symbolic_links:bool
}
struct DirWithList {
    name:String,
    entries:Vec<DirEntryData>
}

fn gen_current_direntrydata() -> DirEntryData{
    return DirEntryData{    name:".".to_string(),
    is_symlink:false,
    path:Path::new(".").display().to_string(),
        size:0,is_dir:true,
        modified_data:Duration::new(0,0)
    }
    }
    fn gen_parent_direntrydata() -> DirEntryData{
        return DirEntryData{   
            is_symlink:false,
            is_dir:true,
            name:"..".to_string(),
            path:Path::new("..").display().to_string(),
            size:0,
            modified_data:Duration::new(0,0)
        }
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

    //COMMAND_SETTINGS
    let command_settings=CommandSettings{
        is_all_excluding_dot:is_all_excluding_dot,
        is_long:is_long,
        is_recursive:is_recursive,
        is_all:is_all,
        do_not_follow_symbolic_links:do_not_follow_symbolic_links
    };

    list_directory(path,&command_settings );
}



fn get_entries(path: &str, commandsettings:&CommandSettings) -> Vec<DirWithList>  {
    let mut entries :Vec<DirWithList>=Vec::new(); 

    add_entries(&mut entries, path,&commandsettings);
   
    //add . and .. if is_all
    if commandsettings.is_all {
        add_current_and_parent(&mut entries);
    }
        return entries;
}

fn add_current_and_parent(entries :&mut Vec<DirWithList>){
    let current_dir_entry=gen_current_direntrydata();
    let parent_dir_entry=gen_parent_direntrydata();
    
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

fn get_direntrydata(entry:DirEntry,command_settings:&CommandSettings) -> DirEntryData{
    let mut name=entry.file_name().to_str().unwrap().to_string();
    let path =entry.path().display().to_string(); 
    let is_dir=is_directory(entry.path());
    let is_exe=is_executable(entry.path());
    let is_fifo=is_fifo(entry.path());
    let metadata = if command_settings.do_not_follow_symbolic_links {
        fs::symlink_metadata(&path) // Do not follow symbolic links
    } else {
        entry.metadata() // Follow symbolic links
    };

    if let Ok(metadata) = metadata {
        let file_type = metadata.file_type();
        let is_symlink = file_type.is_symlink();

        let size = metadata.len();
        let modified_time = metadata.modified().unwrap();
        let modified_time = modified_time.duration_since(std::time::UNIX_EPOCH).unwrap();
        
        if command_settings.do_not_follow_symbolic_links{
        if is_symlink {
            name=format!("{}@",name)
        }else if is_dir{
            name=format!("{}/",name)
        }else if is_exe{
            name=format!("{}*",name)
        }else if is_fifo{
            name=format!("{}|",name)
        }
    }
        return DirEntryData{
            name:name,
            path:path,
            is_dir:is_dir,
            is_symlink:is_symlink,
            modified_data:modified_time,
            size:size
        };
    
    } else {
        println!("Could not read metadata for {}", entry.path().display());
        return DirEntryData{
            name:name,
            path:path,
            is_dir:is_dir,
            is_symlink:false,
            modified_data:Duration::new(0,0),
            size:0
        };
    }
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
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
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

fn add_entries(entries_vec: &mut Vec<DirWithList>,path: &str, command_settings:&CommandSettings) {
    let mut direntries_data_vec:Vec<DirEntryData>=Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        let collected: Vec<_> = entries.filter_map(Result::ok).collect();
        for entry in collected {        
                if should_display(&entry, command_settings) {
                    let p=entry.path();
                    let name = entry.file_name();
                    let pstr=p.to_str().unwrap();
                    // println!("Add {}",pstr);

                    let data=get_direntrydata(entry,command_settings);
                    direntries_data_vec.push(data);
                    if command_settings.is_recursive && p.is_dir() {
                        // Avoiding infinite loop by not re-listing '.' or '..'
                        if ( name != "." && name != "..") {
                            let mut dirs :Vec<DirWithList>=Vec::new(); 
                            add_entries(&mut dirs, pstr, command_settings);
                            for dir in dirs{
                                entries_vec.push(dir);
                            }
                        }
                    }
                }
        }
        let d :DirWithList=DirWithList{
            name:path.to_string(),
            entries:direntries_data_vec,
        };
     
        entries_vec.push(d);
    } else {
        eprintln!("Failed to read directory: {}", path);
    }
    

}



fn list_directory(path: &str,commandsettings:&CommandSettings) {
    let mut dirs :Vec<DirWithList>=get_entries(path, commandsettings); 
 
    // Sort entries alphabetically and case-insensitively within each directory list
    for dir in &mut dirs {
        dir.entries.sort_by_key(|entry| entry.name.to_lowercase());
    }

      // Sort entries alphabetically and case-insensitively within each directory list
    dirs.sort_by_key(|dir| dir.name.to_lowercase());
    


    if !commandsettings.is_recursive {
        // Access the only element immutably
        let de = &dirs[0];
        display_entries(&de.entries, commandsettings);
   
    } else {
        // Access all elements immutably
        for dir in dirs {
            if dir.name!="." {
                println!("\n{}:", dir.name);
            }
            display_entries(&dir.entries, commandsettings);
        }
    }


}

fn should_display(entry: &DirEntry, commandsettings:&CommandSettings) -> bool {
//    let entryname=entry.file_name();
  //  let name=entryname.to_str().unwrap();
    if commandsettings.is_all {
      //  let show=true;
        // println!("Check {} {}",name,show);
        return true;
    }else if commandsettings.is_all_excluding_dot {
        let show=!entry.file_name().to_str().map_or(false, |s| s.starts_with('.'));
        // println!("Check {} {}",name,show);
        return  show;
    }
    return  !entry.file_name().to_str().map_or(false, |s| s.starts_with('.'))
}

fn display_entries(entries: &[DirEntryData], commandsettings:&CommandSettings) {
    if commandsettings.is_long {
        for entry in entries{
            println!("{:<10} {:<20} {}", entry.size, format!("{:?}", entry.modified_data), entry.path);
            return; 
        }  
    }
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
                    if let Some(entry) = entries.get(col * rows + row) { // Calculate correct index for column-first ordering
                        print!("{:<width$}\t", entry.name, width = max_len);
                    }
                }
                println!(); // End the line after each row
            }
            /* 
        if let Some((width, _)) = dimensions() {
            let mut max_len = 0;
            for entry in entries {
                let len = entry.name.len();
                if len > max_len {
                    max_len = len;
                }
            }
            let columns = width / (max_len + 8); // +8 for padding and tab space
            let mut col = 0;
            for entry in entries {
                print!("{:<width$}\t", entry.name, width = max_len);
                col += 1;
                if col >= columns {
                    println!();
                    col = 0;
                }
            }
            if col > 0 {
                println!();
            }*/
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