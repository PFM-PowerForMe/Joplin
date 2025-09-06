# Joplin

[![PFM-Upstream-Sync](https://github.com/PFM-PowerForMe/Joplin/actions/workflows/fork-sync.yml/badge.svg)](https://github.com/PFM-PowerForMe/Joplin/actions/workflows/fork-sync.yml)

## 简介
安全笔记和待办事项应用程序，具有Windows、MacOS、Linux、Android和IOS的同步功能。 

## 如何部署?

1. 前置条件:
```shell
mkdir -p /etc/containers/systemd
podman network create deploy
```

2. 部署
```shell
mkdir -p /opt/podman-data/env
nvim /opt/podman-data/env/joplin.env
```

```
APP_NAME=Joplin Server -- example.com
APP_PORT=3210
APP_BASE_URL=https://example.com

USER_DATA_AUTO_DELETE_ENABLED=true
USER_DATA_AUTO_DELETE_AFTER_DAYS=90
EVENTS_AUTO_DELETE_ENABLED=true
EVENTS_AUTO_DELETE_AFTER_DAYS=30

DB_CLIENT=pg
POSTGRES_PASSWORD=passwd
POSTGRES_DATABASE=joplin_db
POSTGRES_USER=joplin_db_user
POSTGRES_PORT=5432
POSTGRES_HOST=localhost

STORAGE_DRIVER=Type=Database; Mode=ReadAndWrite
STORAGE_DRIVER_FALLBACK=Type=Filesystem; Path=/data

SIGNUP_ENABLED=false
ACCOUNT_TYPES_ENABLED=true


MAILER_ENABLED=true
MAILER_HOST=
MAILER_PORT=587
MAILER_SECURITY=MailerSecurity.Starttls
MAILER_AUTH_USER=
MAILER_AUTH_PASSWORD=
MAILER_NOREPLY_NAME=
MAILER_NOREPLY_EMAIL=
```
---
```shell
nvim /etc/containers/systemd/joplin.container
```

```
# /etc/containers/systemd/joplin.container

[Unit]
Description=The joplin container
Wants=network-online.target
After=network-online.target

[Container]
AutoUpdate=registry
ContainerName=joplin
Timezone=local
Network=deploy
EnvironmentFile=/opt/podman-data/env/joplin.env
Volume=/opt/podman-data/joplin/data:/data
PublishPort=127.0.0.1:6003:3210
Image=ghcr.io/pfm-powerforme/joplin:latest

[Service]
Restart==on-failure
RestartSec=30s
StartLimitInterval=30
TimeoutStartSec=900
TimeoutStopSec=70

[Install]
WantedBy=multi-user.target default.target
```