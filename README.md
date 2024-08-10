# Yaisd
## Super simple initialization system

The system is a simple daemon that starts when your system starts and runs the commands you set in multithreaded mode. You can use it with systemd.

## Installation

```bash
curl -fsSL https://raw.githubusercontent.com/leonovk/yaisd/master/install.sh | bash
```

You can enter the following command to verify that the installation was successful.

```bash
yaisd --version
```

Command --help will offer you a list of possible commands

If the yaisd command was not found, most likely you need to enter a new path in the settings of your .bash_profile (or similar)

`echo 'export PATH=${executable_folder}:\$PATH' >> .bash_profile`

In this case, an example of such a command will be shown to you at the end of the installation script.

## Usage

yaisd will try to find the configuration file in this path:

`$HOME/.config/yaisd/config`

In this file you need to specify the necessary commands for launching. Each new line, a new command.

command `yaisd` start deamon.

You can create a systemd unit so that the daemon starts every time at startup.

For example:

```
[Unit]
Description=Yaisd service

[Service]
Environment=HOME=/home/your_username
ExecStart=/home/your_username/bin/yaisd
KillMode=process

[Install]
WantedBy=multi-user.target
```
