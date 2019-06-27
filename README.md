# Bundlr

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
from: oh-my-zsh

# Bundles
rupa/z@master
desyncr/key-bindings
reem/watch@master
zsh-users/zsh-syntax-highlighting@0.5.*
#zdharma/fast-syntax-highlighting
zsh-users/zsh-autosuggestions@master # do not use develop
zsh-users/zsh-completions
psprint/zsh-navigation-tools
#desyncr/gitr@develop
psprint/zsh-select                   # required by zsh-ctrlp
garybernhardt/selecta@v0.0.*
desyncr/zsh-ctrlp@develop
desyncr/zsh-icdiff
#Vifon/deer deer
hlissner/zsh-autopair
supercrabtree/k@0.*
per-directory-history
LuRsT/hr
ddollar/git-utils                    # various git utilities
git://repo.or.cz/safe-rm.git         # Just kill me
docker
arzzen/calc.plugin.zsh

use: oh-my-zsh {
    desyncr/auto-ls
}
```
