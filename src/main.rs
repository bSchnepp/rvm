#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use  rocket_contrib::serve::StaticFiles;

#[get("/hello")]
fn index() -> String
{
	"Hello, world".to_string()
}


fn main() 
{
	let res = concat!(env!("CARGO_MANIFEST_DIR"), "/res");
	println!("{}", res);
	rocket::ignite()
		.mount("/", routes![index])
		.mount("/", StaticFiles::from(res))
		.launch();
	
}
