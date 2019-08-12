# ServerTools
Copyright (c) 2019 Richard Benson (rbenson@pdx.edu)

This program is a web application written in rust that will use the winapi crate to query the state of windows servers (using windows API calls) and display various operating metrics of those servers.
Including a list of services and their status.

This web application is built on the nightly toolchain and is using nightly-2019-08-10

## Build and Run
1. Install rust for windows https://www.rust-lang.org/tools/install
2. Install postgres 11 for windows https://www.postgresql.org/download/windows/
3. Download ServerTools crate https://github.com/rkbjunior/ServerTools and rename to ServerTools
4. Download the wmi-rs fork https://github.com/rkbjunior/wmi-rs and rename to wmi-rs

At this point you should have directories project\wmi-rs and project\ServerTools

5. Navigate to the ServerTools direcotry from the command line
6. Run the command: rustup override set nightly-2019-08-10

7. Edit the .env file with a new username/passord of your choosing. (use the ones from when you installed postgres)
8. Install the diesel cli by running the command: "cargo install diesel_cli --no-default-features --features postgres"

If you get a linker error, its probably because diesel cant find your postgres drivers. You may need to edit your PATH envrionment variable
Add the following paths (unlesss you installed postgres in a non default location):

C:\Program Files\PostgreSQL\11\lib
C:\Program Files\PostgreSQL\11\bin

Retry step 8, but first close and reopen your command prompt as it will not recognize the new paths unless you do.

9. Run "diesel setup" This will create the database and run the existing migrations. Ensure your .env file has the correct username and password for postgres.

10. Build this program with `cargo build --bin main`. You can run the program with `cargo run --bin main`.

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