use super::post;
use super::rocket;
use lib::models;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn server_tools_base_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn server_tools_add_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client.get("/add").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn server_tools_css_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client.get("/static/css/servertools.css").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn server_tools_js_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client.get("/static/js/main.js").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn server_tools_templates_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client.get("/static/templates/base.html.tera").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn server_tools_post_add_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client
        .post("/add")
        .body("servername=Test&ipaddress=test")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn server_tools_post_remove_route() {
    let client = Client::new(rocket()).expect("Valid rocket instance");
    let response = client.post("/remove/Test").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn validate_add_server_true_test() {
    let ns = models::NewServer {
        servername: "SQL01".to_string(),
        ip_address: "192.168.0.01".to_string(),
    };

    let b = post::validate_add_server(&ns);

    assert_eq!(b, true);
}

#[test]
fn validate_add_server_false_test() {
    let ns = models::NewServer {
        servername: "".to_string(),
        ip_address: "192.168.0.01".to_string(),
    };

    let b = post::validate_add_server(&ns);

    assert_eq!(b, false);
}

#[test]
fn test_mb_conversion() {
    let mut num = 354.0;
    num = lib::convert_memory_units(num, "MB".to_string());
    assert_eq!(num, 0.345703125);
}

#[test]
fn test_gb_conversion() {
    let mut num = 354000.0;
    num = lib::convert_memory_units(num, "GB".to_string());
    assert_eq!(num, 0.3376007080078125);
}

#[test]
fn test_rounddwn_decimals() {
    let mut num = 4564.564897654;
    num = lib::round_decimals(num, 2);
    assert_eq!(num, 4564.56);
}

#[test]
fn test_roundup_decimals() {
    let mut num = 4564.565897654;
    num = lib::round_decimals(num, 2);
    assert_eq!(num, 4564.57);
}
