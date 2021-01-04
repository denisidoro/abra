# abra <img src="https://user-images.githubusercontent.com/3226564/65362934-b4432500-dbdf-11e9-8f75-815fbc5cbf8f.png" alt="icon" height="28px"/> [![Actions Status](https://github.com/denisidoro/abra/workflows/Tests/badge.svg)](https://github.com/denisidoro/abra/actions) ![GitHub release](https://img.shields.io/github/v/release/denisidoro/abra?include_prereleases)
  
A tool that makes data sharing between terminal windows easy.

**abra** can be used for displaying info about the current working directory, for splitting stdout and stderr and much more.

![Terminal demo](TODO)

Table of contents
-----------------

   * [Installation](#installation)
   * [Usage](#usage)
   * [Cheatsheet repositories](#cheatsheet-repositories)
   * [Cheatsheet syntax](#cheatsheet-syntax)
   * [Customization](#customization)
   * [More info](#more-info)
   * [Trying out online](#trying-out-online)
   * [Similar tools](#similar-tools)
   * [Etymology](#etymology)

Installation
------------

**abra** can be installed with the following package managers:

[![Packaging status](https://repology.org/badge/vertical-allrepos/abra.svg)](https://repology.org/project/abra/versions)

The recommended way to install **abra** is by running: 
```sh
brew install abra
```

If `brew` isn't available, you can check [alternative install instructions](docs/installation.md).

Basic concept
-----------------
- `abra rx --channel <name>` will listen to text sent to the *<name>* channel
- `echo foo | abra tx --channel <name>` will listen send text to the *<name>* channel

Common use cases
-----------------

### File tree sidebar

Since this is a very common use case, **abra** provides a hook for you. 

If you call the following...

```sh
# If you use bash
eval "$(abra hook bash)" # in your .bashrc file

# If you use zsh
eval "$(abra hook zsh)" # in your .zshrc file

Then you can open a new terminal window and call `abra rx --channel '
# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

The full syntax and examples can be found [here](docs/cheatsheet_syntax.md).

Customization
-------------

You can:
- [change colors](docs/customization.md#changing-colors)
- [resize columns](docs/customization.md#resizing-columns)
- [change how search is performed](docs/customization.md#overriding-fzf-options)

More info
---------

Please run the following command to read more about all possible options:
```sh
abra --help
```

In addition, please check the [/docs](docs) folder.

Trying out online
-----------------

If you don't have access to a Unix shell at the moment and you want to live preview **abra**, head to [this playground](https://www.katacoda.com/denisidoro/scenarios/abra). It'll start a docker container with instructions for you to install and use the tool. Note: login required.

Similar tools
-------------

There are many similar projects out there ([beavr](https://github.com/denisidoro/beavr), [bro](https://github.com/hubsmoke/bro), [cheat](https://github.com/cheat/cheat), [cheat.sh](https://github.com/chubin/cheat.sh), [cmdmenu](https://github.com/amacfie/cmdmenu), [eg](https://github.com/srsudar/eg), [how2](https://github.com/santinic/how2), [howdoi](https://github.com/gleitz/howdoi) and [tldr](https://github.com/tldr-pages/tldr), to name a few).

They are excellent projects, but **abra** remains unique in the following ways:
- it's natural to write cheatsheets tailored to your needs
- arguments are neither hardcoded nor a simple template

Etymology
---------

In [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time), [abra](https://zelda.gamepedia.com/abra) is a character that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and progress in his quest.
