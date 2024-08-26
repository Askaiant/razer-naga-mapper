# Razer Naga X remapper

Fork of [razer-naga-2014-key-remap](https://github.com/jpodeszwik/razer-naga-2014-key-remap/tree/master/)
Thank you, it was a great inspiration 

Currently works only with:
- Razer Naga X

## Install
Clone the repo

Build it with `cargo build --release`, 
and copy the built executable

With `sudo cp ./target/release/razer-naga-mapper /usr/local/bin/
`

## Running
You can run it as root `sudo /usr/local/bin/razer-naga-mapper`

### Systemd
Or you can create service file in `sudo touch /etc/systemd/system/razer-naga-mapper.service`

Paste in the following code snippet

```
[Unit]
Description=Razer Naga startup script

[Service]
Type=idle
ExecStart=/usr/local/bin/razer-naga-mapper

[Install]
WantedBy=multi-user.target
```

Lastly start the service by running `sudo systemctl start razer-naga-mapper.service`

You can enable it so it runs on startup `sudo systemctl enable razer-naga-mapper.service`
