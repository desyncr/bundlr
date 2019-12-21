# Bundlr [![Build Status](https://travis-ci.org/desyncr/bundlr.svg?branch=master)](https://travis-ci.org/desyncr/bundlr)

Fun project to play with Rust. It's supposely a package manager for sh/bash/zsh etc shells (scripts, binary tools, etc).
It supports a custom configuration language (see below).

# Features

- Semantic versioning
- Version locking
- Supports multiple sources (github, bitbucket, ...)
- Supports conditionals (ie, if osx, if linux etc)
- ..

# TODOs

Check out `TODO` file for details.

# Configuration

```
LuRsT/hr                # Load hr package from LuRsT @ master branch
rupa/z@master           # Another way to specify branch/version

zsh-users/zsh-syntax-highlighting@0.5.*     # Use 0.5.* version
desyncr/zsh-ctrlp@develop                   # Use develop branch

supercrabtree/k@0.*                         # Use 0.*

git://repo.or.cz/safe-rm.git                # Load package from full repo URL

use: oh-my-zsh {
    docker                                  # Load this package from oh-my-zsh repository
}

```
