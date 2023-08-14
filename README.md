# ld-grep: A Cray-aware tool to grep the ld search path

Find yourself asking: Where the heck is do we keep MKL? And why is the linker not finding it? This tool is here to help:

```
blaschke@perlmutter:login38:~> ld-grep --help
Perform regex searches on LD_LIBRARY_PATH and other common library locations

Usage: ld-grep [OPTIONS] <REGEX>

Arguments:
  <REGEX>  Regex to match against files in search paths

Options:
  -c, --use-cray <use_cray>  Interrogate Cray Compiler Wrappers for additional paths [default: true] [possible values: true, false]
      --cc-cmd <CC>          cray compiler wrapper command to check for libraries [default: cc]
  -h, --help                 Print help
  -V, --version              Print version
```

E.g.:

```
blaschke@perlmutter:login38:~> ld-grep mkl
blaschke@perlmutter:login38:~> ml load PrgEnv-intel
blaschke@perlmutter:login38:~> ld-grep mkl
/opt/intel/oneapi/mkl/2023.1.0/lib/intel64/libmkl_avx.so.2
...
```

But wait! There's more! `ld-grep` analyses the Cray Compiler Wrapper to find additional library locations which are introduced by the compiler wrappers.

It does this by parsing `<cray-compiler-cmd> --cray-print-opts=all`.
If you want to specify which compiler command (inlcuding options) to use here, set the `-cc-cmd=` flag.
