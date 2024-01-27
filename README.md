# How it works? (Still in progress)

Dotmanager will look for a directory called **dotfiles** located
at the user home directory ("~/") where there should
be a file **".dotmanager"** witch contains all the paths
to the dotfiles.

The ".dotmanager" file is used both to keep track of what
files / directories to update as well as where to put
the files / directories on a fresh instalation.

So, for example if in the ".dotmanager" there is an entry
`~/.config/nvim`

Dotmanager will know on a fresh system to put the `~/dotfiles/nvim`
at `~/.config/nvim` and also to copy `~/.config/nvim` to `~/dotfiles/nvim`
when you want to update the config.
