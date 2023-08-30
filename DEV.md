# Dev Notes
The cyberdeck frontend is a [svelte](https://svelte.dev/) frontend and an [axum](https://github.com/tokio-rs/axum) backend. Along with servering the frontend app the backend also manages a job queue we refer to as `fixer`. The backend process job request sent from the frontend and sends them to the `fixer`. After the job is completed it gets the information from the `fixer`, performs post processing, saves the data to a sqlite database, and makes it available for the frontend to use.

# Install
You need both [rust](https://www.rust-lang.org/) and [nodejs](https://nodejs.org/en) installed.

## Rust 
For `rust` we recommend installing [rustup](https://www.rust-lang.org/tools/install)

## Node JS
For `nmp` and `nodejs` we recommend using a [Node version manager](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). We use 18.17.1 LTS.

# Directory Layout
To build the application from source you have to first build the frontend svelte app. Then build the rust server.

## Frontend
Located in the `ui` directory. The first time you have to run `npm install`.

Use `npm run build` from inside the `ui` directory to build the app that is served by the backend. This should make automatically make `./ui/dist` which contains the compiled frontend that is served by the backend.

## Backend
Once the frontend is built you can run run `cargo run` from the top level `cyberdeck` directory to build and run the server.

# Project Layout
The project is split between a svelte js frontend and axum rust backend.

## Frontend
The front end is in the `ui` directory.
`ui`
|--public `static assests like images and css`
|  | favicon.ico 
|  | global.css `has a couple rules for formating containers.`
|
|--src `the main code`
|  |--component `reusable components`
|  |  |--Navbar `The nav bar at the top of the app`
|  |  |` Navbar has code to handle login/loggedout states and to handle mobile browsers`
|  |
|  |--js `js functions used on pages`
|  |  |--auth `functions to login, logout, and check if the current cookie is valid`
|  |  |--fetch `functions to fetch from the rust api`
|  |  |` To check what the endpoint returns look for the endpoint in the backend src/routes/mod.rs file`
|  |  |--store `global store that just stores the user if they are logged in`
|  | 
|  |--pages `The main UI for the app`
|  |  |--Apicheck `Simple page to test sending api requests using token auth.`
|  |  |--Login `Login forum`
|  |  |--Logout `Logs the user out`
|  |  |--Secure `Test page to only return data from secure endpoint while logged in`
|  |
|  |--app.css `main css theme for the app`
|  |--main.js `loads the svelte app. Shouldn't need to touch this`
|  |--App.svelte `Entry point for the App.
|
|--Everything else (index.html, package.json, ect..)
|` These you shouldn't have to touch unless adding to package.json`


## Backend
The main rust code is located in the `src` directory.
