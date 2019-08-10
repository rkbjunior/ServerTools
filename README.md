# ServerTools
Copyright (c) 2019 Richard Benson (rbenson@pdx.edu)

This program is a web application written in rust that will use the winapi crate to query the state of windows servers (using windows API calls) and display various operating metrics of those servers. Additionally it will provide functions to restart specific services and web applications.

## Build and Run
Build this program with `cargo build`. You can run the program with `cargo run`.

Additional information on how to operate the program to come.

## Mentions
This project makes use of the chartjs open source project.
- by https://github.com/chartjs/Chart.js/graphs/contributors - https://www.chartjs.org/

The radial guage chart is an extension of chart js and was compiled from the chartjs-chart-radial-guague git repository.
- by pandameister - https://github.com/pandameister/chartjs-chart-radial-gauge

This project uses jQuery which can be downloaded from jquery.org

## License
This program is licensed under the "MIT license". Please see the file `LICENSE` in the source distribution of this software for license terms.