# Reddit Mailer

Reddit mailer consists of two tools:

- `rm-watcher` watches a list of subreddits and inserts the stories it discovers
  into a SQLite database.
- `rm-sender` reads all stories captured by `rm-watcher` and sends them to an
  email address.

Used together, these tools allow you to monitor a list of subreddits and email
yourself the top stories from those subreddits at an interval that you can
configure using cron or systemd.

## Installation

Ensure you have a recent Rust compiler (version 1.56 or above recommended),
clone this repository, and install the binaries with:

```
$ cd path/to/reddit-mailer
$ cargo install --path .
```

## Configuration

### Step 1: Get an API token from reddit

[Create a new app on reddit](https://ssl.reddit.com/prefs/apps/). Make sure you
mark it as a personal use script. Note the client ID and client secret for the
new app.

### Step 2: Configure Reddit Mailer

1. Create a new directory called `reddit-mailer` in the default configuration
   directory for your OS.
       - On Linux, this is `~/.config/` and on macOS it's
         `~/Library/Application Support/`.
2. Copy `config.json` from the `sample_configs/` directory into the newly
   created `reddit-mailer` directory.
3. Edit `config.json` and replace all placeholder keys with the correct
   values. The names of the configuration keys should be self-explanatory.

You will now be able to successfully run `rm-watcher` to download stories from
reddit and `rm-sender` to email them to yourself. Try them out at an interactive
shell to ensure they're working as expected.

### Step 3: Configre Reddit Mailer to run at an interval using systemd

If you're on Linux, you can automate Reddit Mailer using systemd. In the
`sample_configs/` directory you will find a set of systemd unit files that you
can copy into your distribution's default directory for systemd.

You can edit these files to configure how often `rm-watcher` should download new
stories, and how often `rm-sender` will mail them to you. After configuring the
intervals, enable the services and have fun!

## License

This software is distributed under the terms of the MIT license. See
[LICENSE](LICENSE) for more information.
