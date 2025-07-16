A collection of details and scripts to run [amaru](https://github.com/pragma-org/amaru) and Raspberry Pis.

# Build

## On RPI

Building directly on the real machine is always an option but might require [tweaks](#tweaks) and time!

```shell
git clone https://github.com/pragma-org/amaru
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install -y libclang-dev 
cargo build --release # 1 hour on RPI5
```

## Cross building

Cross building allow to pre-create binaries for RPI on different (more powerful) machine with different architecture.
Note that cross-building is probably better on a linux environment.

```shell
cross build --target=armv7-unknown-linux-gnueabihf --release
```

# Running

Regular `amaru` commands can be used to run on an RPI. Note that it's probably a good idea to start with a fresh amaru db. Running `bootstrap` (to start from a `cardano-node` snapshot) will either be pretty slow or crash (on Pi zero).

# Tweaks

Some RPIs require specific configuration to be able to run `amaru`.

## Pi ZERO

Pi zero do not have any swap by default. Couple with the lower amount of ram (512MB) it won't run `amaru` OOB.


```shell
# Increase swap

sudo dphys-swapfile swapoff
sudo vi /etc/dphys-swapfile
# edit `CONF_SWAPSIZE=100` to `CONF_SWAPSIZE=1024`
sudo dphys-swapfile setup
sudo dphys-swapfile swapon
```

# TODO

* [ ] create full custom images via GH actions
* [ ] add amaru binary and conf
* [ ] provide images for RPI 4/5/Zero2
* [ ] allow to easily access amaru dbs
* [ ] create script for [inky](https://github.com/pimoroni/inky)
* [ ] experiment with [pisugar](https://github.com/PiSugar/PiSugar/)
* [ ] pretty print nice 3d enclosure
* [ ] make sure `amaru-doctor` runs smoothly on RPI5 with a touch screen