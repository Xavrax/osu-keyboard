### Configure that for your environment:

# set path for avrdude
export AVRDUDE=/usr/bin/avrdude

# set path for avrdude config
export AVRDUDE_CONF=/etc/avrdude.conf

### Build:

# build program
cargo build -Z build-std=core --target avr-atmega328p.json --release

# make .hex file
avr-objcopy -j .text -j .data -O ihex target/avr-atmega328p/release/osu-keyboard.elf osu-keyboard.hex

# uploud to arduino
$AVRDUDE -C $AVRDUDE_CONF -v -patmega328p -cusbasp -Pusb -U flash:w:osu-keyboard.hex