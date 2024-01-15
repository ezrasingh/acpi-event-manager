# Why Did I Make This

## Preface

For those who don't know; Original Equipment Manufacturers (OEM i.e, NVIDIA, HP, Dell, etc) may or may not extend Linux support for their native components (e.g Power Management IC, Fingerprint Reader, etc). What this means is that if you bought a OEM laptop and installed Linux on it there is no guarantee your OS will be able to use all the features of your machine.

I should say rather, support varies. Either the company uses weird proprietary chips and don't care for the Linux market. Or, they do try to support Linux but cant keep up with product releases, patches and updates.

## The Problem

In my case I done goof'd and bought an HP (it slipped my mind in the moment) and after installing my Ubuntu environment I come to the **horror** that my screen brightness was not working, and I could barely see anything(even in broad daylight). So I did some internet sleuthing and followed [thread](https://askubuntu.com/questions/1230937/ubuntu-20-04-brightness-adjust-not-working) after [thread](https://askubuntu.com/questions/1406420/cant-change-brightness-on-version-22-04).

Eventually I found out I could enable this flag in my boot loader:

```bash
GRUB_CMDLINE_LINUX_DEFAULT="quiet splash acpi_backlight=vendor"
```

I enabled it hoping this would work and well it did but ... sorta. Now my brightness was all the way up and nothing I did could change it (hotkey didn't work, GUI didn't work).To get by, I just fiddled with `xrandr` and found a moderate brightness then added this line to my shell startup:

```bash
# ~/.zshrc
xrandr --output eDP --brightness 1.5
```

Because I often prefer to work late night, I would have to constantly switch and toggle `xrandr`` to control my brightness. Which eventually (after weeks of this) started to get annoying.

## Research & Tinkering

So one morning, I made a large cup of coffee, sat down in my living room and started digging to find a better (ideally permanent) solution. Eventually, I learned that if I modified the files in `/sys/class/backlight/acpi_video0` directory, I could control low level display values.

> The name `acpi_video0` is [not necessarily generic](https://wiki.archlinux.org/title/backlight), this _depends on your graphics card model_

For illustration purposes my directory looked like this:

```shell
❯ tre /sys/class/backlight/acpi_video0
/sys/class/backlight/acpi_video0
├── uevent
├── actual_brightness
├── bl_power
├── brightness
├── power/
│   ├── runtime_active_time
│   ├── runtime_active_kids
│   ├── runtime_usage
│   ├── runtime_status
│   ├── autosuspend_delay_ms
│   ├── async
│   ├── runtime_suspended_time
│   ├── runtime_enabled
│   └── control
├── device/
├── type
├── scale
├── subsystem/
└── max_brightness
```

The files of interest to me were: `brightness`, `actual_brightness` and `max_brightness`.

Each of these files store an ASCII encoded integer, just change the number and save the file. ACPI will read the values stored in them to control hardware components.

Whoever thought of having two separate variables for brightness I hate them, the time [wasted](https://bbs.archlinux.org/viewtopic.php?id=174991) because of this was traumatic, And in the end, neither `brightness` or `actual_brightness` worked. However, `max_brightness` _did_ seem to have an affect and `xrandr` was still working.

So I thought okay I'll just:

1. Make a bash script to toggle brightness
2. Read the values of `brightness` and `max_brightness`
3. Increment and modify the `brightness` value
4. Call `xrandr`
5. Call it a day

_However_, the input to `xrandr --brightness` is a `float` and ... `floats` + bash [don't mix](https://stackoverflow.com/questions/12722095/how-do-i-use-floating-point-arithmetic-in-bash). And I didn't feel like having to install an entire package just to make a one line call to `xrandr`. Using a programming language at this point felt justified so I went with Rust because:

1. Programs compile to a single binary (i.e no interpreter)
2. Natural choice for system level programming
3. `cargo` was already installed on my machine

The icing on the cake was that I also discovered how to create custom ACPI event handlers by adding a config files to `/etc/acpi/events/`. Meaning I could run a command/script every time a specified event fired (i.e pressed hotkey, low power, etc).

To find the exact ACPI event associated with my brightness up/down hotkeys I ran `sudo acpi_listen` to log all events into my shell, which for me looked like this:

```shell
❯ sudo acpi_listen
video/brightnessdown BRTDN 00000087 00000000
video/brightnessup BRTUP 00000086 00000000
```

## Implementing The Solution

As you can probably tell by now, keeping track of all these details is getting out of hand. So I decided to consolidate the config into a single file that serves as an abstraction for all these settings. Using Rust made implementing the solution so smooth, it goes like this:

1. You run `ls /sys/class/backlight` & `sudo acpi_listen`
2. You create/update [`config.toml`](config.toml) with data from step 1
3. Run the CLI
4. Rust generates the event handler config
5. Rust applies it to ACPI by storing in `/etc/acpi/events/`
6. Rust restart the ACPI daemon

And that's why I created the [ACPI Event Manager](https://github.com/ezrasingh/acpi-event-manager).

Now, I at least automated the ACPI configuration onto my system. Maybe in the future I'll update the CLI to **auto detect** the ACPI events and what not with a `setup` sub-command.
