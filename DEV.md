# Dev Notes
The cyberdeck frontend is a [svelte](https://svelte.dev/) frontend and an [axum](https://github.com/tokio-rs/axum) backend. Along with servering the frontend app the backend also manages a job queue we refer to as `fixer`. The backend process job request sent from the frontend and sends them to the `fixer`. After the job is completed it gets the information from the `fixer`, performs post processing, saves the data to a sqlite database, and makes it available for the frontend to use.

# Install
You need both [rust](https://www.rust-lang.org/) and [nodejs](https://nodejs.org/en) installed.

## Rust 
For `rust` we recommend installing [rustup](https://www.rust-lang.org/tools/install)

## Node JS
For `nmp` and `nodejs` we recommend using a [Node version manager](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). We use 18.17.1 LTS.

# Setup
To build the application from source you have to first build the frontend svelte app. Then build the rust server.

## Frontend
Located in the `ui` directory. The first time you have to run `npm install`.

Use `npm run build` from inside the `ui` directory to build the app that is served by the backend. This should make automatically make `./ui/dist` which contains the compiled frontend that is served by the backend.

## Backend
Once the frontend is built you can run run `cargo run` from the top level `cyberdeck` directory to build and run the server.