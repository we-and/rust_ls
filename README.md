# rust_ls 


## Roadmap
| Command   | Coverage                             | Test |
|-----------|--------------------------------------|------|
| ls        | 100%                                 | Yes  |
| ls path   | 100%                                 | Yes  |
| ls -A     | 100%                                 | Yes  |
| ls -C     | 100%                                 | Fail (output width)  |
| ls -R     | 100%                                 | Yes  |
| ls -h     | 80% (need to exclude if -H and -L)  |      |
| ls -H     | TODO                                 |      |
| ls -L     | TODO                                 |      |
| ls -S     | 100%                                 | Yes  |
| ls -a     | 100%                                 | Yes  |
| ls -c     | Differs on macOS                     |      |
| ls -d     | 100%                                 |  Yes    |
| ls -f     | TODO                                 |      |
| ls -g     | TODO                                 |  Yes    |
| ls -i     | 100%                                 | Yes  |
| ls -k     | TODO                                 |      |
| ls -l     | 100%                                 | Yes  |
| ls -m     | 100%                                 | Yes  |
| ls -n     | 100%                                 | Yes  |
| ls -o     | 100%                                 |   Yes   |
| ls -p     | 100%                                 |    Yes  |
| ls -q     | 100%                                 |      |
| ls -r     | 100%                                 |      |
| ls -s     | 100%                                 |   Yes   |
| ls -t     | 100%                                 |      |
| ls -u     | TODO                                 |      |
| ls -x     | 100%                                 |   Fail (output width)   |
| ls -1     | 100%                                 |  Yes    |

## Examples
### -l
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
-rw-r--r--  1 jd  staff    287 Apr 19 10:42 README.md
-rw-r--r--  1 jd  staff  32797 Apr 19 08:26 lsaR.unix
-rw-r--r--  1 jd  staff  86815 Apr 19 08:26 lsaRl.unix
-rw-r--r--  1 jd  staff  22248 Apr 18 22:54 rust.out
lrwxr-xr-x  1 jd  staff      8 Apr 19 09:12 rust.out.symlink -> rust.out
drwxr-xr-x  3 jd  staff     96 Apr 18 18:42 src
drwxr-xr-x@ 5 jd  staff    160 Apr 18 18:42 target
-rw-r--r--  1 jd  staff  22248 Apr 18 22:54 unix.out

```
### -m
```
./target/debug/rust_ls -m           
Cargo.lock, Cargo.toml, README.md, outputs, src, target, test_environments, tests
```

### -r
```
./target/debug/rust_ls -r 
tests                   target                  outputs                 Cargo.toml       
test_environments       src                     README.md               Cargo.lock
```
### -s
```
./target/debug/rust_ls -s
total 48
32 Cargo.lock             8 README.md              0 src                    0 test_environments     
 8 Cargo.toml             0 outputs                0 target                 0 tests
```


### -p
```
./target/debug/rust_ls -p
Cargo.lock              README.md               src/                    test_environments/
Cargo.toml              outputs/                target/                 tests/
```


J Dumont
