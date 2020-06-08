## bear-cli

This is a small application for me to experiment with, and get a feel for, Rust. This isn't anything serious, just something fun for me to work on in some down time. The application itself is a command line interface for the [Bear](https://bear.app/) note taking for Mac. Right now it has very limited functionality.

This really grew out of me wanting to randomly open up a note in Bear, so that's all that is implemented at the moment.

### Installation

Download the generated `bear-cli` binary from the [releases page](https://github.com/jerkeeler/bear-cli/releases). Store the binary in a directory of your choosing that is added to your `$PATH` environment variable (i.e. `/usr/local/bin`). That's it!

### Commands

#### random

Usage:

```bash
bear-cli random
```

Chooses a random note and opens it. Useful for spaced repetition and revisiting of notes that you may have forgotten about.
