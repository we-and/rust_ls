use clap::{App, Arg};
use std::fs::File;
use std::io::{Write, Read, BufReader, BufWriter};

use std::fs::{OpenOptions};
use std::io::{self, Seek, SeekFrom,  Cursor};
use std::path::Path;

struct ArHeader {
    name: String,
    timestamp: String,
    owner_id: String,
    group_id: String,
    mode: String,
    size: usize,

}


fn main() {
    let matches = App::new("Rust ar")
        .version("0.1.0")
        .author("J Dumont")
        .about("Implements an ar command in Rust")
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
    







    // Parsing logic for each option
    if matches.is_present("A") {
        
    }else if matches.is_present("C") {
    
    }else if matches.is_present("F") {
        
    }else if matches.is_present("H") {
    
    }else if matches.is_present("L") {
    
    }else if matches.is_present("R") {
    
    }else if matches.is_present("S") {
    
    }else if matches.is_present("a") {
    
    }else if matches.is_present("c") {
    
    }else if matches.is_present("d") {
    
    }else if matches.is_present("g") {
    
    }else if matches.is_present("f") {
    
    } else if matches.is_present("i") {
    
    } else if matches.is_present("i") {
    
    } else if matches.is_present("i") {
    
    } else if matches.is_present("k") {
    
    } else if matches.is_present("l") {
    
    } else if matches.is_present("m") {
    
    } else if matches.is_present("n") {
    
    } else if matches.is_present("o") {
    
    } else if matches.is_present("p") {
    
    } else if matches.is_present("q") {
    
    } else if matches.is_present("r") {
    } else if matches.is_present("s") {
    
    } else if matches.is_present("t") {
    
    } else if matches.is_present("u") {
    
    } else if matches.is_present("x") {
    
    } else if matches.is_present("1") {
    

    } 

    // Add further logic here for other commands
}
