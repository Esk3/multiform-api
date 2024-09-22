use maud::Render;
use poem::handler;
use util::MaudTemplate;

mod pages;
pub mod util;

pub fn web_router() -> poem::Route {
    poem::Route::new().at("/", poem::get(hello_world))
}
#[handler]
fn hello_world() -> poem::web::Html<maud::Markup> {
    let body = maud::html! {
        h1 {"my h1"}
        p {"hi from html rendering"}
    };
    let template = MaudTemplate::new(body, "hello world");
    poem::web::Html(template.render())
}
