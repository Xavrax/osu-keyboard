# :warning: Work in progress! :warning:
# osu-keyboard

Osu! keyboard project written in rust for my own but feel free to fork or use it.

## How to start

All you need to start play with this project:
- rustup
- arduino uno (for development purposes)
- avr-atmega328p and other electronic stuff (to finish project)

You can get rustup [here](acquire)!

Now you have to install nightly version of rust's toolchain:
```shell
rustup toolchain install nightly
```

Now you can compile project using `build.sh` script.

> :warning: Due to `bullet_in` problem you need to install nightly-2021-01-07 toolchain. :warning:

```shell
rustup toolchain install nightly-2021-01-07
```

Now if you would like to deploy your project to arduino/atmega you have to change 
variables in `deploy-atmega328p.sh` and use it.

Have fun :)!