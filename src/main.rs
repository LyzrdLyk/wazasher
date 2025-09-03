#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;
mod redirector;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // See `Rocket.toml` and `Cargo.toml` for TLS configuration.
    // Run `./private/gen_certs.sh` to generate a CA and key pairs.
    rocket::build()
        .mount("/", routes![hello])
        .attach(redirector::Redirector { port: 3000 })
}
