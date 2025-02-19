# railmap.arbjerg.dev
This repository builds a bare-bones website to show the places I've traveled by rail, according to Träwelling.

* This is currently hardcoded to my username (`freya`) at the top of `src/status.rs`.
* This crate panics if `TRAEWELLING_BEARER_TOKEN` is not set.
* The resulting site is written to the `./out` directory. Currently it is a single HTML file.
* Buses, planes, and taxis are filtered out, as are private statuses.
* Build with `cargo run`
