Contest Programming Skeleton
============================

A quick directory structure for contest programming :smile:

What should be where?
---------------------

The directory structure is as follows,
- `src` directory contains all the soltions. The solution's name should be `<name>.cpp` (It only compiles single c++ files for now).

- `inputs` directory contains local test cases. The filename should be `<name>.in`. This file is going to piped into the program via `stdin`.

- The `Makefile` is used as the build system. It is used via gnu `make`. Honestly I rarely touch the `Makefile`.

- `.editorconfig` file is for editorconfig plugin. Go [here](http://editorconfig.org/) for more info.

How do I use it?
----------------
To compile a solution you should write
```bash
$ make <solution>
```

It will compile the solution and keep it in `bin` folder which will be created via the makefile

If you want to get rid of the binaries run,
```bash
$ make clean
```
This will delete the `bin` folder.

Example
-------

For example an example solution is given which adds to number. I called it `add`.
Thus, the binary name will be `add` (path `bin/add`) and the source file will be `add.cpp` (path `src/add.cpp`).
The input file will be named `add.in` (path `inputs/add.in`).

To run this solution issue the command `make add`. This will pipe the input file and print the result on console :smile:

Experiment and Ideas
--------------------
`Make` it your own :wink:

