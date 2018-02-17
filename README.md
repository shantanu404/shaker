Contest Programming Skeleton
============================

A quick directory structure for contest programming :smile:

What should be where?
---------------------

The directory structure is as follows,
- `src` directory contains all the solutions.

- `inputs` directory contains local test cases. The filename should be `<name>.in`. This file is going to piped into the program via `stdin`.

- `expected` directory contains the expected results for the respective test cases in `inputs` directory.

- The `checker` is used as the build system. It is a python3 script.

- `.editorconfig` file is for editorconfig plugin. Go [here](http://editorconfig.org/) for more info.

How do I use it?
----------------
To compile and run a file you should write
```bash
$ ./checker <solution>
```

It will compile the solution and keep it in `bin` folder which will be created via the `checker`

If you provide another argument `--expect` to the command like this,
```bash
$ ./checker <solution> --expect
```

then it will match the results of the solution to the expected result in `expected/<solution>.txt`

If you want to get rid of the binaries run,
```bash
$ ./checker --clean
```
This will delete the `bin` folder.

Example
-------

For example an example solution is given which adds to number. I called it `add`.
Thus, the binary name will be `add` (path `bin/add`) and the source file will be `add.cpp` (path `src/add.cpp`)
Note that I have also put some examples in other languages supported by the script.

The input file will be named `add.in` (path `inputs/add.in`).

To run this solution issue the command `./checker add.cpp`. This will pipe the input file and print the result on console

Issuing `./checker add.cpp --expect` will compare the output stream to `expected/add.txt`

You can try other examples like..
```bash
$ ./checker add.go
$ ./checker add.py
$ ./checker add.c
$ ./checker add.rs
```

### You are welcome to make it your own, suggest ideas and contribute to make it better :smile:
