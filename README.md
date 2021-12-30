# chess.rs
chess.rs is a chess game developed in rust with SDL2 ! This project has been developed in order to learn Rust through self thought projects !

## Roadmap:

* Make a board where you can drag and drop pieces, and a player cannot play the ennemy pieces.  ✅
* Make a turn by turn system. ✅
* Include the FEN Notation to initialize a board. ✅
* Made every movement rules for pieces ✅ (en passant check: ❌)

## Prerequisites

The project is built with following libraries, tools and utilitaries:

* [Rust](https://www.rust-lang.org/)
* [SDL2](https://www.libsdl.org/index.php)
* [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/)
* [SDL2_image](https://www.libsdl.org/projects/SDL_image/)
* [SDL2_gfx](https://www.libsdl.org/index.php)
* [SDL2_mixer](https://www.libsdl.org/projects/SDL_mixer/)

### Install Rust

As it is built with Rust, in order to build the project, you need to have the compiler installed !
As a reminder, here is how you install it:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install SDL tools

* On MacOS:

Assuming you have homebrew installed, please type the following command in your terminal:
```bash
brew install sdl2 sdl2_gfx sdl2_image sdl2_ttf sdl2_mixer
````

* On Ubuntu:

In order to install the SDL tools on your Ubuntu, please type the following command in your terminal:
```bash
sudo apt install libsdl2-2.0-0 libsdl2-gfx-1.0-0 libsdl2-image-2.0-0 libsdl2-ttf-2.0-0 libsdl2-mixer-2.0-0
```

## Usage

First, you need to clone the repository. If you have an ssh key registered, please type following command:
```bash
git clone git@github.com:LilianSchall/chess.rs.git && cd chess.rs/src
```

* Build the project:

With Rust and all its tools with it installed, type the following command:
```bash
cargo build
```

* Run the project:

Simply type the following command:
```bash
cargo run
```


## Contacts:

If you have any question about the project, feel free to ask me on social media !

* [Twitter](https://twitter.com/lilixns)
* [Instagram](https://www.instagram.com/404lilian/)
* [LinkedIn](https://www.linkedin.com/in/lilian-schall-456338206/)

