#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
pub struct Doc {
    doc_id: String
}

#[get("/")]
fn index() -> &'static str {
    "欢迎使用 Hyoka Search Engine!"
}
#[put("/put/doc/", format = "application/json", data = "<doc_id>")]
fn put_doc(doc_id: Json<Doc>) -> String {

}
fn main() {
    rocket::ignite().mount("/",
                           routes![index,
                           put_doc])
        .launch();
}
