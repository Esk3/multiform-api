use poem::handler;

pub fn web_router() -> poem::Route {
    poem::Route::new().at("/", poem::get(hello_world))
}
#[handler]
fn hello_world() -> poem::web::Html<String> {
    dbg!("in here!");
    poem::web::Html("hello".to_string())
}
