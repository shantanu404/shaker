Shaker
======

Shaker is a simple _checker_ script for competetive programmers. It compiles and checks your solution with only one command and provides
a little structure to your workspace.

What should be where?
---------------------

The directory structure is as follows,
- `src` directory contains all the solutions.

- `inputs` directory contains local test cases. The filename should be `<name>.in`. This file is going to piped into the program via `stdin`.

- `expected` directory contains the expected results for the respective test cases in `inputs` directory.

- The `shaker` is used as the build system. It is a python3 script.

- `.editorconfig` file is for editorconfig plugin. Go [here](http://editorconfig.org/) for more info.

How do I use it?
----------------
To compile and run a file you should write
```bash
$ ./shaker <solution>
```

It will compile the solution and keep it in `bin` folder which will be created via the `shaker`

If you provide another argument `--expect` to the command like this,
```bash
$ ./shaker <solution> --expect
```

then it will match the results of the solution to the expected result in `expected/<solution>.txt`

If you want to get rid of the binaries run,
```bash
$ ./shaker --clean
```
This will delete the `bin` folder.

Example
-------

For example an example solution is given which adds to number. I called it `add`.
Thus, the binary name will be `add` (path `bin/add`) and the source file will be `add.cpp` (path `src/add.cpp`)
Note that I have also put some examples in other languages supported by the script.

The input file will be named `add.in` (path `inputs/add.in`).

To run this solution issue the command `./shaker add.cpp`. This will pipe the input file and print the result on console

Issuing `./shaker add.cpp --expect` will compare the output stream to `expected/add.txt`

You can try other examples like..
```bash
$ ./shaker add.go
$ ./shaker add.py
$ ./shaker add.c
$ ./shaker add.rs
```

If a program crashes the script will print the error and exit.
```bash
$ ./shaker crash.py
Traceback....
...
FAILURE
```

### You are welcome to make it your own, suggest ideas and contribute to make it better :smile:
