# ez-renamer

## Description

Ez-renamer or in short ezr is a CLI tool written in Rust for batch renaming files. It's purposefuly not very featureful because it's meant to be used for just some quick renaming.

```
ezr [FLAGS] [OPTIONS] [file-match]
```

## Instalation

```sh
cargo install ezr
```

You need to have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

## Installing from source

```bash
git clone https://github.com/krawieck/ez-renamer
cd ez-renamer
cargo install
```

## Args

```
<file-match>
        regular expression for files that should be renamed [default: .]
```

## Flags

```
    --dont-cleanup
        By default ez-renamer removes multiple spaces (cleans up)
        after it's done. This flag stops him from doing that

-h, --help
        Prints help information

-e, --include-ext
        Includes extensions in renaming process

    --include-dirs
        include directories in renaming process

-q
        Program is much quieter, it's recommended
        only if you know what you're doing

        -q results in program just asking if u wanna proceed, and
        -qq results in program not letting anything into stdout

-r, --recursive
        recursively goes through directories

-V, --version
        Prints version information

-y
        confirms the rename, recomended only if you know what you're doing
```

## Options

```
-d, --delete <delete>
        deletes this phrase(s) from names

        example:

        ezr -d "[WEBRip] [720p] [YTS.AM]"

        "Green Book (2018) [WEBRip] [720p] [YTS.AM]" -> "Green Book (2018)"
    --dir <dir>
        directory where should this program look for files
-s, --fix-spaces <fix-spaces>
        whatever you give is replaced by space (but only single chars)

        example:

        `--fix-spaces="_"` results in:

        "the_office_[720p]_[x265]" -> "the office [720p] [x265]"
-t, --rmtags <remove-tags>
        remove tags, they're usually inside [] or (). e.g. -s "() []"

        Syntax for this argument should be '<opening bracket><closing bracket> <repeat>'

        example:

        ezr -s "[] ()"

        "Mind Field S03E02 (2018) [1080p] [x265] [YIFY].mkv" -> "Mind Field S03E02.mkv"
    --trim-left-after <trim-left-after>
        Trim after the given sequence to the left.

        example:

        ezr --trim-left-with Mind

        "[HorribleSubs] Mind Field S03E02.mkv" -> "Mind Field S03E02.mkv"
    --trim-left-with <trim-left-with>
        Trim with the given sequence to the left.

        example:

        ezr --trim-left-with ubs]

        "[HorribleSubs] Mind Field S03E02.mkv" -> "Mind Field S03E02.mkv"
    --trim-right-after <trim-right-after>
        Trim after the given sequence to the right

        example:

        ezr --trim-right-after [1080p]

        "Mind Field S03E02 [1080p] [x265] [YIFY].mkv" -> "Mind Field S03E02 [1080p].mkv"
    --trim-right-with <trim-right-with>
        Trim with the given sequence to the right

        example:

        ezr --trim-right-with [1080p]

        "Mind Field S03E02 [1080p] [x265] [YIFY].mkv" -> "Mind Field S03E02 .mkv"
```
