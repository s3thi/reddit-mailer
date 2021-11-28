# Reddit Mailer

Reddit Mailer is a set of tools that allow you to monitor a list of subreddits
and email yourself the top stories from those subreddits at an interval.

## Installation

Reddit Mailer is built with Rust. Ensure you have a recent Rust compiler
(version 1.56 or above recommended), clone this repository, and install the
binaries with:

```
cargo install --path .
```

## Setup

Reddit mailer consists of two binaries:

- `rm-watcher` watches a list of subreddits and inserts the stories it discovers
  into a SQLite database.
- `rm-sender` reads all captured stories, ranks them according to an algorithm,
  marks them as old, and sends them to you as an HTML email message.

### Step 1: Get an API token from reddit

[Create a new app on reddit](https://ssl.reddit.com/prefs/apps/). Make sure you
mark it as a personal use script. Then copy the client ID and client secret that
reddit give you.

### Step 2: Configure Reddit Mailer

1. Create a new directory called `reddit-mailer` in the default configuration
   directory for your OS. On Linux, this is `~/.config/` and on macOS it's
   `~/Library/Application Support/`.
2. Copy `config.json` from the `sample_configs/` directory into the newly
   created `reddit-mailer` directory.
3. Edit `config.json` and replace all placeholder keys with the correct
   values. The names of the configuration keys are self-explanatory.

You should now be able to successfully run `rm-watcher` to download stories from
reddit and `rm-sender` to email them to yourself.

### Step 3: Set up Reddit Mailer to run using systemd

If you're on Linux, you can automate Reddit Mailer using systemd. In the
`sample_configs/` directory you will find a set of systemd unit files that you
can copy into your distribution's default directory for systemd.

You can edit these files to configure how often `rm-watcher` should download new
stories, and how often `rm-sender` will mail them to you.
