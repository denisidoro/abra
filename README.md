# abra [![Actions Status](https://github.com/denisidoro/abra/workflows/Tests/badge.svg)](https://github.com/denisidoro/abra/actions) ![GitHub release](https://img.shields.io/github/v/release/denisidoro/abra?include_prereleases)
  
A tool that makes data sharing between terminal windows easy.

**abra** can be used for displaying info about the current working directory, for splitting stdout and stderr and much more.

![File tree demo](https://user-images.githubusercontent.com/3226564/103572068-c2878400-4eaa-11eb-86e1-651748a2dcd5.gif)

Table of contents
-----------------

   * [Installation](#installation)
   * [Basic concepts](#basic-concepts)
   * [Common use cases](#common-use-cases)
   * [Advantages over named pipes](#advantages-over-named-pipes)
   * [Forcing colorized output](#forcing-colorized-output)
   * [Similar tools](#similar-tools)
   * [Etymology](#etymology)

Installation
------------

The recommended way to install **abra** is by running: 
```sh
brew install denisidoro/tools/abra
```

You can also run:
```sh
cargo install abra-cli
```

If these package managers aren't available, you can download a pre-compiled binary [here](https://github.com/denisidoro/abra/releases/latest) and extract it to your `$PATH`.

Basic concepts
-----------------
- **abra** is built over Unix sockets
- it can publish and subscribe to channels, manipulating text as necessary
- no terminal multiplexers are necessary

Common use cases
-----------------

Some **abra** calls are quite verbose, so the use of [aliases](https://github.com/denisidoro/abra/blob/master/shell/aliases.bash) is recommended. 

### File tree sidebar

Since this is a very common use case, **abra** provides a hook for you. 

If you call the following...

```sh
eval "$(abra hook bash)" # If you use bash, add this to ~/.bashrc
eval "$(abra hook zsh)" # If you use zsh, add this to ~/.zshrc
```

Then you can open a new terminal window and call `abra rx --channel pwd --cmd 'ls {}'`.

Whenever you `cd` into a directory, the sidebar will reflect the changes.

### Split stdout and stderr into different windows

Let's say that you want to run some tests but errors should appear in a different window.

You can use anonymous pipes with **abra** for that purpose:

![Split demo](https://user-images.githubusercontent.com/3226564/103569522-383d2100-4ea6-11eb-8deb-c8450d8d66a9.png)

The commands are:
```sh
abra rx --channel test_out # window 1
abra rx --channel test_err # window 2
cargo test > >(abra tx --channel test_out) 2> >(abra tx --channel test_err) # window 3
```

### Filter some output lines

Let's say you want to see the contents of a file in a window but show only the lines that contain "foo" in another window:

```sh
abra rx --channel filter --cmd 'echo "{}" | grep foo' # window 1
cat myfile.txt |& tee >(abra tx --channel filter) # window 2
```

Forcing colorized output
-------------

Some CLIs will detect that they are being piped and will hide color information by default. 

To circumvent this, each CLI may offer different paramenters: `--color=always` and `export COLORTERM=truecolor` are some examples.

In some cases, you need to trick an application into thinking its stdout is a terminal, not a pipe. For these cases you can call `abra faketty --cmd '<your command>'`.


Advantages over named pipes
-------------

In theory, you could run the following to achieve similar results:
```sh
mkfifo tmp
tail -f tmp
echo foo > tmp # in another window
```

That said:
- with **abra** you don't need to worry about creating/removing named pipes
- `echo foo > tmp` is blocking in case `tmp` isn't open for reading
   - `abra tx` will terminate immediately if there's no `abra rx` process
- you can have many `abra rx` windows reacting to the same `abra tx` call
- **abra** is cross-platform
   - to correctly create temporary named pipes you need to write platform-specific code

Similar tools
-------------

- [tmux-sidebar](https://github.com/tmux-plugins/tmux-sidebar)

Etymology
---------

[Abra](https://bulbapedia.bulbagarden.net/wiki/Abra_(Pok%C3%A9mon)) is a [Pokémon](https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon) who is able to [teleport](https://bulbapedia.bulbagarden.net/wiki/Teleport_(move)).
