# rskm

Rust SSH Key Manager — a CLI tool for managing SSH keys.

## Installation

```bash
cargo install --path .
```

## RSKM_HOME

All keys and configuration are stored in the `RSKM_HOME` directory.

Default: `~/.rskm`

To override, set the environment variable:

```bash
export RSKM_HOME=/path/to/custom/dir
```

Directory structure:

```
~/.rskm/
├── rskm.toml   # config (default_key_type)
└── keys/       # SSH keys
```

## Usage

### `init`

Initializes `RSKM_HOME`. Must be run before any other command.

```bash
rskm init
```

### `create`

Creates a new SSH key. Defaults to `ed25519`.

```bash
rskm create <key_name>
rskm create <key_name> -t <key_type>
```

Supported types: `ed25519`, `ecdsa`, `rsa`, `xmss`

### `rename`

Renames a key (both private and public files).

```bash
rskm rename <key_name> <new_name>
```

### `delete`

Deletes a key (both private and public files).

```bash
rskm delete <key_name>
```

### `list`

Lists all keys.

```bash
rskm list
```

### `show`

Prints the public key.

```bash
rskm show <key_name>
```

### `add`

Adds a key to `ssh-agent`.

```bash
rskm add <key_name>
```

### `remove`

Removes a key from `ssh-agent`.

```bash
rskm remove <key_name>
```

### `destroy`

Deletes `RSKM_HOME` along with all keys. Prompts for confirmation.

```bash
rskm destroy
rskm destroy --yes   # skip confirmation
```

## TODO

- [ ] `~/.ssh/config` management (hosts)
- [ ] `list --loaded` — show only keys currently loaded in the agent
- [ ] Passphrase-protected key creation
- [ ] `xmss` behind a feature flag (non-standard type, requires a specially built `ssh-keygen`)
- [ ] `export` command — copy public key to clipboard
- [ ] Installation script / distro packages

## License

[LICENSE](LICENSE)
