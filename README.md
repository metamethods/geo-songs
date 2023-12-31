# geo-songs
Custom Geometry Dash song server fully written in Rust!

## Features
- Allows access to NG Songs while custom songs are also available
- Rust, so its very safe

## Prebuilt Binaries
All of the prebuilt binaries will be located in the [actions tab](https://github.com/metamethods/geo-songs/actions), and look for the latest workflow with a checkmark. Click on it, and you'll find all of the built artifacts!

## Building
### Perquisites
- [Rust](https://rustup.rs/)

### Building
1. Clone the repository:
```bash
git clone https://github.com/metamethods/geo-songs.git
``` 

2. Build the project:
```bash
cargo build # Depending on what system you are using, cargo will build the binary for it. Its kinda hard to cross build other systems without having the other system lmao
```

3. The binary will be located in `target/release/geo-songs` or `target/release/geo-songs.exe`
4. Profit

## Usage
__Run the binary__, not that hard lol, but here's the usage anyways:
1. Find where your __Geometry Dash__ executable is located
2. You'll have to open the file in a `hex editor` (For example) or a `text editor` <sub>I just prefer the hex editor as it doesn't look ugly when doing some stuff</sub>
3. Find the string `https://www.boomlings.com/database/getGJSongInfo.php` and replace it with `http://localhost:8000/song_data` (Or whatever port you're using, but __8000__ should be the default port on rocket.rs) __IMPORTANT:__ When replacing the strings, they MUST be the exact same size, you can see here for example:

![Imagine not able to see the image](/images/replace.png)

4. Save the file and run the binary, or run it from Steam
5. Profit (again)

### Adding a song
When wanting to add a new song, you'll have to do it in this format:
<sub>If you don't see the songs directory, open the program just once, it'll generate one for you!</sub>
```
songs
- my-very-cool-song.mp3
- my-very-cool-song.yaml
```

The `.yml` file contains the song's metadata, and the `id` for the song. Heres a template to start you off with:
```yml
id: 1 # Note: When using the id in geometry dash, it'll be negative to differentiate from NG songs
name: My Very Cool Song
file: my-very-cool-song.mp3
author: a very cool person
```

After all of that, you can input the id (In negative form i.e. if the Id of the song is 1, then you should put -1), and it'll show up!

## Super Duper Important Thing
You __MUST__ have the program open at all times, or geometry dash will not be able to load any of the songs. You can figure out how to make it run in the background and stuff.

## Contributing
If you want to contribute, feel free to open a PR! I'll be happy to review it and merge it if it looks good!
