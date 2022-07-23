#[macro_use] extern crate rocket;

use rocket::Request;
use rocket_dyn_templates::{Template, context};
use serde_json::Value;
use rocket::fs::{FileServer, relative};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        //title: "Hello",
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", context! {
        //title: "Hello",
    })
}

#[get("/projects")]
pub fn projects() -> Template {
    // open the projects.json file and parse it into a json object
    let projects = std::fs::read_to_string("info/projects.json").unwrap();
    let projects: Value = serde_json::from_str(&projects).unwrap();
    Template::render("projects", context! {
        projects: projects,
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, projects])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}