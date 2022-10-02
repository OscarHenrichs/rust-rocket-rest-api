#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world2"
}

fn main() {
    const PATH: &str = "data.sqlite";

    {
        let db_connection = rusqlite::Connection::open(PATH).unwrap();

        db_connection
            .execute(
                "CREATE TABLE IF NOT EXISTS TEST (id integer primary key);",
                [],
            )
            .unwrap();
    }

    rocket::ignite().mount("/", routes![index]).launch();
}
