# ctags-query

A tool to query ctags with simple boolean expressions.

For example, to find locations tagged with `foo`, `bar`, but _not_ `buzz`:

```sh
$ ctags-query 'foo&bar&!buzz'
path/to/fileX:10
path/to/fileY:50
path/to/fileZ:165
```

I'm not sure how useful this is for a regular ctags file, because that wasn't my [motivation](#Motivation), but hopefully it can be useful for other uses as well.


## Motivation

[Vimwiki](https://github.com/vimwiki/vimwiki) is a fantastic application for keeping notes. In my notes, I like to leverage tags as much as possible, but until now, have only been able to query a single tag at once with `:VimwikiSearchTags` or `:FzfTags`. Inspired by [orgmode's tag search](https://orgmode.org/manual/Matching-tags-and-properties.html#Match-syntax), I set out to build something similar to help me find stuff in my vimwiki notes.

This repository is the actual implementation of that functionality, while the vim specific code lives at [vim-tagquery](https://github.com/matt-snider/vim-tagquery).

## Releasing

Currently I do this sort of manually:

* Update version in Cargo.toml, rebuild, and commit ("Bump version to x.y.z")
* Run script to tag and build binaries: `./prep-release.sh x.y.z`
* Push: `git push && git push --tags`
* Within GitHub, make a new release and upload the binaries in `release-tmp/`

