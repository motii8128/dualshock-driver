# dualshock-driver
connect dualshock4 by hidapi in Rust

# Build Test
[![Rust](https://github.com/motii8128/dualshock-driver/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/dualshock-driver/actions/workflows/rust.yml)

# Before Using
Add udev rules
```
cd /etc/udev/rules.d
```
```
sudo touch ./99-hidraw-permissions.rules
```

execute the following command to rewrite the contents of the file
```
sudo vi ./99-hidraw-permissions.rules
```

contents
```:99-hidraw-permissions.rules
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0660", GROUP="plugdev"
```

After
```
reboot
```
