use super::rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
#[ignore = "need to get a test db up and running"]
fn test_auth() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/auth");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}

#[test]
#[ignore = "need to get a test db up and running"]
fn test_change_pw() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/auth/change_pw");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}

#[test]
#[ignore = "need to get a test db up and running"]
fn test_sign_in() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/auth/sign_in");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}

#[test]
#[ignore = "need to get a test db up and running"]
fn test_params() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.get("/params?email=test@test.com&api=testApi");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}

#[test]
#[ignore = "need to get a test db up and running"]
fn test_param_options() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.options("/auth");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}

#[test]
#[ignore = "need to get a test db up and running"]
fn test_items_sync() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/items/sync");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}

#[test]
#[ignore = "WORKS locally, but need to get a test db up and running"]
fn test_bad_request() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/auth");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some("{\"errors\":[\"Bad request\"]}".into())
    );
}

#[test]
#[ignore = "WORKS locally, but need to get a test db up and running"]
fn test_unauthorized() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/items/sync");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some("{\"errors\":[\"Unauthorized\"]}".into())
    );
}

#[test]
#[ignore = "WORKS locally, but need to get a test db up and running"]
fn test_not_found() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.get("/notFound");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some("{\"errors\":[\"Not found\"]}".into())
    );
}

#[test]
#[ignore = "need to sign in and then post data that makes it throw up"]
fn test_server_error() {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.get("/auth");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.headers().get_one("X-Special").is_some());
    assert_eq!(response.body_string(), Some("Expected Body".into()));
}
