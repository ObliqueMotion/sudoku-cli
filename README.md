# sudoku-cli

---
## About

* A command-line tool for solving sudoku puzzles. 
* If your input is a file path, `sudoku-cli` will read from the file. Otherwise it treats the string as input.  
* `sudoku-cli` reads the first 81 non-whitespace characters from the input and fills each row from left to right, 
starting with the top row.
* `sudoku-cli` can output results to a file. The directory must already exist.

---
## Commands

`sudoku-cli` has 5 sub-commands each with their own configurable options:
* `sudoku-cli find-one`
    * `-i=<value> | --input=<value>`
    * `-t=<value> | --threads=<value>`
    * `-o=<value> | --output=<value>`
    * `-c | --compact`
* `sudoku-cli find-all`
    * `-i=<value> | --input=<value>`
    * `-t=<value> | --threads=<value>`
    * `-o=<value> | --output=<value>`
    * `-c | --compact`
* `sudoku-cli watch-one`
    * `-i=<value> | --input=<value>`
    * `-m=<value> | --ms-per-frame=<value>`
* `sudoku-cli watch-all`
    * `-i=<value> | --input=<value>`
    * `-m=<value> | --ms-per-frame=<value>`
* `sudoku-cli count-all`
    * `-i=<value> | --input=<value>`
    * `-t=<value> | --threads=<value>`
    
---
 ## Examples
 
---
 ### Find One
 
```sudoku-cli find-one --input=".75.....4.1...5.....8.7.........7.......6...1...8.2...3...9.7...5.3.4.........31."```

<img src="https://raw.githubusercontent.com/ObliqueMotion/sudoku-cli/master/images/find-one.png">

---

 ### Find All
 
`sudoku-cli find-all --input=path/to/puzzle --threads=8 --output=solutions.txt --compact`

<img src="https://raw.githubusercontent.com/ObliqueMotion/sudoku-cli/master/images/find-all.png">

---

 ### Watch One
 
`sudoku-cli watch-one --input=path/to/puzzle`

<img src="https://raw.githubusercontent.com/ObliqueMotion/sudoku-cli/master/images/watch-one.gif">

---

 ### Watch All

`sudoku-cli watch-all --input=path/to/puzzle --ms-per-frame=15`

<img src="https://raw.githubusercontent.com/ObliqueMotion/sudoku-cli/master/images/watch-all.gif">

---

 ### Count All

`sudoku-cli count-all --input=path/to/puzzle --threads=8`

<img src="https://raw.githubusercontent.com/ObliqueMotion/sudoku-cli/master/images/count-all.png">

