# rskm

Rust SSH Key Manager — CLI-утилита для управления SSH-ключами.

## Установка

```bash
cargo install --path .
```

## RSKM_HOME

Все ключи и конфигурация хранятся в директории `RSKM_HOME`.

По умолчанию: `~/.rskm`

Чтобы переопределить — задайте переменную окружения:

```bash
export RSKM_HOME=/path/to/custom/dir
```

Структура директории:

```
~/.rskm/
├── rskm.toml   # конфиг (default_key_type)
└── keys/       # SSH-ключи
```

## Использование

### `init`

Инициализирует `RSKM_HOME`. Необходимо выполнить перед использованием остальных команд.

```bash
rskm init
```

### `create`

Создаёт новый SSH-ключ. По умолчанию используется тип `ed25519`.

```bash
rskm create <key_name>
rskm create <key_name> -t <key_type>
```

Поддерживаемые типы: `ed25519`, `ecdsa`, `rsa`, `xmss`

### `rename`

Переименовывает ключ (приватный и публичный файлы).

```bash
rskm rename <key_name> <new_name>
```

### `delete`

Удаляет ключ (приватный и публичный файлы).

```bash
rskm delete <key_name>
```

### `list`

Выводит список всех ключей.

```bash
rskm list
```

### `show`

Выводит публичный ключ.

```bash
rskm show <key_name>
```

### `add`

Добавляет ключ в `ssh-agent`.

```bash
rskm add <key_name>
```

### `remove`

Удаляет ключ из `ssh-agent`.

```bash
rskm remove <key_name>
```

### `destroy`

Удаляет `RSKM_HOME` вместе со всеми ключами. Запрашивает подтверждение.

```bash
rskm destroy
rskm destroy --yes   # без подтверждения
```

## TODO

- [ ] Управление `~/.ssh/config` (хосты)
- [ ] Команда `list --loaded` — показывать только ключи, загруженные в агент
- [ ] Поддержка парольной защиты ключей при создании
- [ ] `xmss` за feature-флагом (нестандартный тип, требует специальной сборки `ssh-keygen`)
- [ ] Команда `export` — копировать публичный ключ в буфер обмена
- [ ] Установочный скрипт / пакеты для дистрибутивов

## Лицензия

[LICENSE](LICENSE)
