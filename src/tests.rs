
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let profiles = [        "ed25519",    ];

    // TODO: Testing doesn't actually read keys since we don't do TLS locally.
    for profile in profiles {
        let config = rocket::Config::figment().select(profile);
        let client = Client::tracked(super::rocket().configure(config)).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
