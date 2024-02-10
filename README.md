# dualshock-driver
connect dualshock4 by hidapi in Rust

# Before Using
Add udev rules
```
cd /etc/udev/rules.d
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
