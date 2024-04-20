use std::ffi::OsString;

use atty::{is, Stream};
pub fn is_printable(c:char)->bool{
    return  c.is_ascii_graphic() || c==' ';
 }
 pub  fn sanitize_filename(name:OsString) -> String{
     return     name.to_string_lossy().chars().map(|c| if is_printable(c){c} else {'?'}).collect();
     }
 
     
     pub fn is_tty_output() -> bool{
       return  atty::is(Stream::Stdout); 
     }