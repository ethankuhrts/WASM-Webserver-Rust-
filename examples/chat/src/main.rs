use std::{collections::HashMap, ops::Range};


use webserver::{Server, ServerInitOptions, Route, macros::get, Templates, HttpResponse};

fn main() {
    // Create a Server Instance using ServerInitOptions
    // Currently only options are IP and PORT
    // Must define as mutable
    let mut server = Server::new(ServerInitOptions {
        ip: String::from("127.0.0.1"),
        port: 9500, // Port can be any 16 bit int
        public_directory: String::from("./res"),
    });

    // Initialize backend server routes
    // e.g. /favicon.ico
    server.init();

    // Here we define the index route
    // Route::new(
    //     [route path e.g. "/" or "/contact"], 
    //     [callback a function returning a String]
    // )
    // callback must return a String

    // Get a mutable reference to the Router
    // The router is behind a Mutex so we must lock it to access it
    let mut router = server.router();

    let index = Route::new("/", || -> HttpResponse {
        let mut response = HttpResponse::new();
        response.set_body(Templates::render("index.html").unwrap());
        return response;
    });

    

    // Register the index route we made
    router.register(index);

    // once all routes are registered, drop the router to free up use of it to run the server
    drop(router);

    // Finally start the server
    server.start()
}