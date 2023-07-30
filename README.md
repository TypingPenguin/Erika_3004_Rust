# Erika_3004_Rust

This is a project inspired by https://github.com/Chaostreff-Potsdam/erika3004/tree/master. It allows to interact with an Erika 3004 over the internet via an ESP32.
This ReadMe will be updated over time.

## State
Currently, the code is in some kind of alpha and is still under development. Do not expect for things to just work.

## Installation
*Currently this has only been tested with the base ESP32*

The first step is to install the development environment. Do this by following the steps described in [The Rust on ESP Book](https://esp-rs.github.io/book/installation/index.html).

Once you have cloned the repo locally, please copy the file `example.cfg.toml` and rename the copy `cfg.toml`. Fill the SSID and Password of your 2.4Ghz WiFi network.

Now you are ready to build and run the code. After connecting your ESP32 via USB to your computer, use the command `cargo run` in the terminal while in the project. This will build the application for you and flash it to the ESP32 if you have followed the previous step.

You should be able to see the logs in the terminal and see if the program has started correctly on the ESP32. It will give you the IP of the user interface (for me it is http://192.168.1.130/).

Make sure you have connected the hardware of the Erika 3004, level shifter and ESP32 correctly as described in the following step.

## Connecting the hardware

//TODO
