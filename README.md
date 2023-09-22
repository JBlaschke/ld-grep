# ld-grep: A Cray-aware tool to grep the ld search path

Find yourself asking: Where the heck is do we keep MKL? And why is the linker
not finding it?  Or maybe the linker complains about a missing symbol, and you
don't know where to look? This tool is here to help:

```
blaschke@perlmutter:login38:~> ld-grep --help
Perform regex searches on LD_LIBRARY_PATH and other common library locations.

Usage: ld-grep [OPTIONS] [REGEX]

Arguments:
  [REGEX]  Regex to match against files in search paths

Options:
  -c, --cc-cmd <CC>  Interrogate Cray Compiler Wrappers for additional paths. CC = command to check for libraries [default: ]
  -s, --symbol       List libraries that define or import the symbol
  -i, --info         List all locations that are searched
  -h, --help         Print help
  -V, --version      Print version
```

`ld-grep` uses a fully-fledged grep, so 
- `mkl` searches for all occurrences for of the letter combination "mkl"
- `^.*\\.so[0-9\\.]*$` searches for all occurences of `.so.<version number>`

`ld-grep` searches the same locations as `ld.so.conf`, the `LD_LIBRARY_PATH`,
and the search paths from the Cray compiler wrappers. By default the Cray
compiler wrappers are not checked unless you specify the `-c` or `-cc-cmd` flag
(cf. [below](#cray-compiler-wrappers)).

The `-i` or `--info` flag will print out the search paths. E.g.:

```
blaschke@perlmutter:login38:~> ld-grep -i
LD-GREP is searching for libraries (and symbols thereing) here: 
    0. /lib64
    1. /lib
    2. /usr/lib64
    3. /usr/lib
    4. /usr/local/lib64
    5. /usr/local/lib
    6. /opt/cray/pe/lib64
    7. /opt/cray/pe/lib64/cce
    8. /opt/cray/xpmem/default/lib64
    9. /opt/esmi/e_smi/lib/
    10. /opt/cray/pe/gcc/11.2.0/snos/lib64
    11. /global/common/software/nersc/pe/llvm/mpich/4.1.1/lib
    12. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/math_libs/11.7/lib64
    13. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/CUPTI/lib64
    14. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/Debugger/lib64
    15. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/nvvm/lib64
    16. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/lib64
    17. /opt/cray/pe/papi/7.0.0.1/lib64
    18. /opt/cray/libfabric/1.15.2.0/lib64
Found 2707 dynamically-linked libaries.
Found 104 statically-linked libaries.
```

## Use Cases

### Find all library files containting a string

For examples, let's find the locations where Intel MKL is installed:

```
blaschke@perlmutter:login38:~> ld-grep mkl
blaschke@perlmutter:login38:~> ml load PrgEnv-intel
blaschke@perlmutter:login38:~> ld-grep mkl
/opt/intel/oneapi/mkl/2023.1.0/lib/intel64/libmkl_avx.so.2
...
```

This example shows that the correct module has to be loaded (in order for the
`LD_LIBRRY_PATH` to be configured).

### Find the location of a library used by the Cray compiler wrappers (but not included in `LD_LIBRARY_PATH`)

Certain Cray modules don't add locations to the `LD_LIBRARY_PATH`, instead
adding them to the Cray compiler wrapper link line. For `libnvidia-ml.so` is one
such library.

By default `ld-grep` does not check the compiler wrappers, so without the `-c`
flag it would only find the version that is part of the OS install:

```
blaschke@perlmutter:login38:~> ld-grep ^/.*/libnvidia-ml.so$
/usr/lib64/libnvidia-ml.so
```

Inserting a compiler command shows which version the compiler wrapper adds:

```
blaschke@perlmutter:login38:~> ld-grep  ^/.*/libnvidia-ml.so$ -c cc
/usr/lib64/libnvidia-ml.so
/opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/lib64/stubs/libnvidia-ml.so
```

## Searching for Symbols Accross Libraries

`ld-grep -s` will search of all definitions and uses of symbols accross its
search path:

```
blaschke@perlmutter:login38:~> ld-grep -c cc -s ^MPI_Comm_get_attr$
...
WEAK   FUNC   DEFAULT define       /opt/cray/pe/lib64/libmpi_gnu_91.so: MPI_Comm_get_attr
GLOBAL FUNC   DEFAULT define       /opt/cray/pe/lib64/lib_pat_mpi.so: MPI_Comm_get_attr
...
GLOBAL FUNC   DEFAULT import       /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/math_libs/11.7/lib64/libcal.so: MPI_Comm_get_attr
...
/opt/cray/pe/mpich/8.1.25/ofi/gnu/9.1/lib/libmpi_gnu_91.a: lib_libmpi_gnu_91_la-comm_get_attr.o MPI_Comm_get_attr
...
```

Note that `ld-grep` searches EVERYWHERE, so you will probably find multiple
install trees where libraries exist which define the symbol that you're looking
for.

The example above shows 3 different ways symbols can be identified by in
dynamically-linked libraries by `ld-grep`:
1. `WEAK   FUNC   DEFAULT define` denotes a [weak symbol](https://en.wikipedia.org/wiki/Weak_symbol) definition
2. `GLOBAL FUNC   DEFAULT define` denotes a strong symbol definition
3. `GLOBAL FUNC   DEFAULT import` denotes a symbol usage (but not definition)

The final line above shows that the symbol also occurs in a statically-linked
library (`libmpi_gnu_91.a`) along with the object file therein
(`lib_libmpi_gnu_91_la-comm_get_attr.o`).

## Cray Compiler Wrappers

`ld-grep` analyses the Cray Compiler Wrapper to find additional library
locations which are introduced by the compiler wrappers.

It does this by parsing `<cray-compiler-cmd> --cray-print-opts=all`.
If you want to specify which compiler command (inlcuding options) to use here,
set the `-cc-cmd=` flag. The argument passed to `-c` or `--cc-cmd=` define the
compiler command. Note that this can include flags.

Generally speaking `-c` is not necessary -- and will break in a `PrgEnv` that
doesn't use the Cray compiler wrappers. Therefore it is not enabled by default.
However, it does expand the `ld-grep` search paths:

```
blaschke@perlmutter:login38:~> ld-grep -i
LD-GREP is searching for libraries (and symbols thereing) here: 
    0. /lib64
    1. /lib
    2. /usr/lib64
    3. /usr/lib
    4. /usr/local/lib64
    5. /usr/local/lib
    6. /opt/cray/pe/lib64
    7. /opt/cray/pe/lib64/cce
    8. /opt/cray/xpmem/default/lib64
    9. /opt/esmi/e_smi/lib/
    10. /opt/cray/pe/gcc/11.2.0/snos/lib64
    11. /global/common/software/nersc/pe/llvm/mpich/4.1.1/lib
    12. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/math_libs/11.7/lib64
    13. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/CUPTI/lib64
    14. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/Debugger/lib64
    15. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/nvvm/lib64
    16. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/lib64
    17. /opt/cray/pe/papi/7.0.0.1/lib64
    18. /opt/cray/libfabric/1.15.2.0/lib64
Found 2707 dynamically-linked libaries.
Found 104 statically-linked libaries.
blaschke@perlmutter:login38:~> ld-grep -i -c cc
LD-GREP is searching for libraries (and symbols thereing) here: 
    0. /lib64
    1. /lib
    2. /usr/lib64
    3. /usr/lib
    4. /usr/local/lib64
    5. /usr/local/lib
    6. /opt/cray/pe/lib64
    7. /opt/cray/pe/lib64/cce
    8. /opt/cray/xpmem/default/lib64
    9. /opt/esmi/e_smi/lib/
    10. /opt/cray/pe/gcc/11.2.0/snos/lib64
    11. /global/common/software/nersc/pe/llvm/mpich/4.1.1/lib
    12. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/math_libs/11.7/lib64
    13. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/CUPTI/lib64
    14. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/Debugger/lib64
    15. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/nvvm/lib64
    16. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/lib64
    17. /opt/cray/pe/papi/7.0.0.1/lib64
    18. /opt/cray/libfabric/1.15.2.0/lib64
    19. /opt/cray/pe/mpich/8.1.25/ofi/gnu/9.1/lib
    20. /opt/cray/pe/mpich/8.1.25/gtl/lib
    21. /opt/cray/pe/libsci/23.02.1.1/GNU/9.1/x86_64/lib
    22. /opt/cray/pe/dsmml/0.2.2/dsmml//lib
    23. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/lib64/stubs
    24. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/lib64
    25. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/nvvm/lib64
    26. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/CUPTI/lib64
    27. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/cuda/11.7/extras/Debugger/lib64
    28. /opt/nvidia/hpc_sdk/Linux_x86_64/22.7/math_libs/11.7/lib64
    29. /opt/cray/xpmem/2.5.2-2.4_3.50__gd0f7936.shasta/lib64
Found 2880 dynamically-linked libaries.
Found 158 statically-linked libaries.
```

Which can be helpful when trying to find libraries only used by the compiler
wrappers ([discussed above](#find-the-location-of-a-library-used-by-the-cray-compiler-wrappers-but-not-included-in-ld_library_path)).

## Common Issues

You can ignore messages like:
```
Error when listing symbols. ELFError Unknown magic: 0x6c20554e47202a2f in /usr/lib64/libc.so
```
which tell you that the given file does not match the usual ELF file format. In
this case the contents was not searched.
