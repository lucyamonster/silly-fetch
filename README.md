# Welcome
This is `silly-fetch` a tiny fetch util for linux.

## Install / Build
To install just run `cargo install --git https://github.com/lucyamonster/silly-fetch.git`

## Config
Currently the only way to configure is using a config file. 
If no file is used a default will be used.
Example config:
```
{
    "layout": [
        "userHostname",
        "divider",
        "os",
        "host",
        "kernel",
        "uptime"
    ],
    "clearTerm": true
}
```
To use the config add `-c <pathToFile><fileName>.json`