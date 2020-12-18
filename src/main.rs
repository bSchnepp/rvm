#![feature(decl_macro)]
#[macro_use] extern crate rocket;

#[get("/hello")]
fn index() -> String
{
	"Hello, world".to_string()
}


fn main() 
{
	rocket::ignite().mount("/", routes![index]).launch();
	println!("Hello, world!");
}
