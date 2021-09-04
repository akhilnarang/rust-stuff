#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world, from rocket"
}

#[get("/some_route")]
fn another_route() -> String {
    "Just another test route".to_string()
}

#[get("/<test_str>")]
fn test_path_parameters(test_str: &str) -> String {
    format!("Test, {}", test_str)
}

#[get("/test/<_>/test")]
fn test_ignore() -> String {
    "Test ignore".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, another_route, test_path_parameters, test_ignore],
    )
}
