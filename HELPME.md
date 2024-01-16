# Help

If you are having trouble setting up the [config.toml](config.toml), here are some tips that helped me identify my system's values.

Here are the system details for the machine I used, along with the command to find them in case your results differ.

```shell
# OS: Ubuntu 23.10 (x86_64)

# ACPI Daemon
❯ acpid -v
acpid-2.0.33

# Display Manager
❯ systemctl status display-manager
● gdm.service - GNOME Display Manager

# XRandR
❯ xrandr -v
xrandr program version       1.5.1
```

## Listing ACPI Devices

To view availble ACPI devices check the following directory `/sys/class/backlight`. For me, it looked like this:

```shell
❯ tre /sys/class/backlight
/sys/class/backlight
└── acpi_video0
```

So in my case I would set `acpi_device = "acpi_video0"` in my [`config.toml`](config.toml)

## Listing Xrandr Displays

By default `xrandr` shows availble displays. For me, it looked like this:

```shell
❯ xrandr
Screen 0: minimum 8 x 8, current 1920 x 1080, maximum 32767 x 32767
HDMI-0 disconnected # ...
eDP-1-0 connected primary # <- we only need this part
   1920x1080    144.00*+  60.02
   1680x1050    144.00
   1280x1024    144.00
# ...
```

So in my case I would set `xrandr_display = "eDP-1-0"`

## Determine ACPI Event Codes

If unsure check by running `sudo acpi_listen` and pressing the corresponding button on your keyboard.

The corresponding event should log onto your shell. You can expect to see something like this (except the comments I added, those are only there to indicate my physical actions):

```shell
❯ sudo acpi_listen
# presses brightess down button
video/brightnessdown BRTDN 00000087 00000000

# presses brightness up button
video/brightnessup BRTUP 00000086 00000000
```
