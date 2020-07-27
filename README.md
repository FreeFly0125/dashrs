# `dash-rs` - Efficient processing of Geometry Dash data

`dash-rs` (spoken 'dashers') is an experimental library related to modelling, and more imporant _processing_ all data formats related to RobTop's 2013 game "Geometry Dash". The long-term goal is to have this crate replace `gdcf_model`, `gdcf_parse` and some parts of `gdrs` in GDCF.

The project is a collaboration with [mgostIH](https://github.com/mgostIH), on whose idea the initial library design is based on and who continues to provide incredibly helpful insights into optimization, Geometry Dash and Rust.

## Goals

The goals for dash-rs are, in order:

+ Zero allocation deserialization for RobTop's HTTP response and local savefile formats, using [serde](https://serde.rs)
+ Accurate modelling of all game related data
+ Efficient serialization routines for RobTop's HTTP response and local save file formats, using serde. 

The benchmark we're trying to beat is parsing and calculating the length in seconds of the level 'Spacial Rend' in `56ms`, achieved on my Laptop's Intel:tm: Core i7-8850H using `gdcf_parse`.

## Disclaimer

I, in no way, claim to know what I'm doing.

## License

This project is licensed under the terms of the MIT license.
