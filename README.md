### rust-ls
An implementation of the unix command ls using Rust.

## Roadmap
| Command  | Coverage |
| ------------- | ------------- |
| ls  | 100%  |
| ls path  | 100%  |
| ls -a   | 100%  |
| ls -A   | 100%  |
| ls -R   | 100%  |
| ls -l   | 100%  |
| ls -h   | 80% (need to exclude if -H and and -L)  |

```
ls -l                                             
total 384
-rw-r--r--  1 jd  staff  12091 Apr 19 10:44 Cargo.lock
-rw-r--r--  1 jd  staff    276 Apr 19 10:44 Cargo.toml
-rw-r--r--  1 jd  staff    287 Apr 19 10:42 README.md
-rw-r--r--  1 jd  staff  32797 Apr 19 08:26 lsaR.unix
-rw-r--r--  1 jd  staff  86815 Apr 19 08:26 lsaRl.unix
-rw-r--r--  1 jd  staff  22248 Apr 18 22:54 rust.out
lrwxr-xr-x  1 jd  staff      8 Apr 19 09:12 rust.out.symlink -> rust.out
drwxr-xr-x  3 jd  staff     96 Apr 18 18:42 src
drwxr-xr-x@ 5 jd  staff    160 Apr 18 18:42 target
-rw-r--r--  1 jd  staff  22248 Apr 18 22:54 unix.out


./target/debug/rust_ls -l  
total 384
-rw-r--r--  1 jd  staff  12091 Apr 19 10:44 Cargo.lock
-rw-r--r--  1 jd  staff    276 Apr 19 10:44 Cargo.toml
-rw-r--r--  1 jd  staff  32797 Apr 19 08:26 lsaR.unix
-rw-r--r--  1 jd  staff  86815 Apr 19 08:26 lsaRl.unix
-rw-r--r--  1 jd  staff    287 Apr 19 10:42 README.md
-rw-r--r--  1 jd  staff  22248 Apr 18 22:54 rust.out
lrwxr-xr-x  1 jd  staff      8 Apr 19 09:12 rust.out.symlink
drwxr-xr-x  3 jd  staff     96 Apr 18 18:42 src
drwxr-xr-x@ 5 jd  staff    160 Apr 18 18:42 target
-rw-r--r--  1 jd  staff  22248 Apr 18 22:54 unix.out
```
