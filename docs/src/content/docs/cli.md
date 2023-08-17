---
title: 'The Eludris CLI'
description: "Everything you'll need to know to be able to use the almighty Eludris CLI!"
order: 2
---

The Eludris CLI is a tool that's meant to handle all the heavy lifting that comes
with initially starting an Eludris instance and managing it until the end of time.

## Installation

You can install the Eludris CLI in a couple of ways, mainly you can either:

- Install it from the [releases page](https://github.com/eludris/eludris/releases/tag/v0.3.2).
- Install it using [Cargo](https://crates.io/crates/eludris).
  ```sh
  cargo install eludris
  ```
- Install it from the [AUR](https://aur.archlinux.org/packages/eludris) on Arch based Linux distributions:
  ```sh
  <your preferred aur helper> -S eludris
  ```

You can also clone the repository and build it from source.

## Usage

If you need any help with using the Eludris CLI at any time you should always check
the help command by running `eludris --help`. This will almost always tell you what
you want to know.

Upon using the CLI for the first time you will get prompted for some config questions
namely about where to put the Eludris instance.

You can find the CLI's current configuration directory using the `eludris conf-dir`
command. You can also overwrite the default directory using the `ELUDRIS_CLI_CONF`
environment variable.

The Eludris CLI automatically reads your `.env` files in your current directory
to facilitate managing multiple instances.

## Commands

Here's a list of the commands the Eludris CLI has along with a few extra notes.

### Deploy

```sh
eludris deploy [--next]
```

This command will start up your Eludris instance using our pre-built Docker images.

Additionally if no instance is already found on your machine it will take you on
a step by step process to create one in your configured Eludris directory.

Using the `--next` flag will make you use the latest development version of Eludris
for your instance instead.

> **Note**
>
> When prompted to choose an editor **_do not_** chose VSCode or any of it's forks
> as this command is ran as root which will really mess up your root partition.
>
> Although you can still pass in a full command to use it you have been warned.

### Stop

```sh
eludris stop
```

This command will stop all the microservices in your instance along with all the
other databases and such.

### Update

```sh
eludris update [--next]
```

This command will update your instance's version of Eludris to the latest available
version on GitHub.

Using the `--next` flag makes you update to the latest development version of ELudris.

### Logs

```sh
eludris logs
```

This command will show you your instance's logs and wait for new ones.

### Static

This is a command group which is meant to help deal with static assets.

#### Add

```sh
eludris static add <path-to-file>
```

This command will add a new static asset to your instance's Effis.

#### Remove

```sh
eludris static remove <name>
```

This command will remove a static asset from your instance's Effis.

### Attachments

This is a command group which is meant to help deal with Effis files in the attachments
bucket.

#### Remove

```sh
eludris attachments remove <id>
```

This command will remove an attachment's file from your instance along with its
database entry to prevent `500` errors.

### Clean

```sh
eludris clean
```

This command will remove your Eludris instance along with all the database files.

> **Note**
>
> This will not clean up the docker images, you can remove those using `docker image rm <image names>*`

### Conf-Dir

```sh
eludris conf-dir
```

This command returns the config directory currently used by the CLI.

Depending on whether you have an `.env` file this can change based on your current directory.

You can also supply the `ELUDRIS_CLI_CONF` environment variable to override the default
configuration directory for your platform.
