# ServerTools
Copyright (c) 2019 Richard Benson (rbenson@pdx.edu)

This program is a web application written in rust that will use the winapi crate to query the state of windows servers (using windows API calls) and display various operating metrics of those servers.
Including a list of services and their status.

This web application is built on the nightly toolchain and is using nightly-2019-08-10

## Build and Run
Build this program with `cargo build --bin main`. You can run the program with `cargo run --bin main`.

This web application uses a modified version of the wmi-rs crate. I created a fork for this crate and modified it to allow remote server connections. You will need to download my fork of the wmi-rs crate
and add it to the directory above the ServerTools Crate.

The web application will only work on windows machines that have the WMI services enabled. You may need to adjust your firewalls to permit WMI traffic.

Be default the root of the webapplication will query the WMI services on the local host. After you register some remote servers on the same local area network,
you should be able to view their wmi information as well, as long as their services are available. 

Once the web application is up and running. Navigate your web browser to http://localhost:8000


## Mentions
This project makes use of the chartjs open source project.
- by https://github.com/chartjs/Chart.js/graphs/contributors - https://www.chartjs.org/

The radial guage chart is an extension of chart js and was compiled from the chartjs-chart-radial-guague git repository.
- by pandameister - https://github.com/pandameister/chartjs-chart-radial-gauge

This project uses jQuery which can be downloaded from jquery.org

I couldn't have built this project without the aid of the eexcellent blog here:
https://notryanb.github.io/rust-blog-series-1.html

And of course the rocket documentation here:
https://rocket.rs/v0.4/guide/


## License
This program is licensed under the "MIT license". Please see the file `LICENSE` in the source distribution of this software for license terms.