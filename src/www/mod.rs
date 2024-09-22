use poem::get;

mod pages;
pub mod util;

pub fn web_router() -> poem::Route {
    poem::Route::new()
        .nest("/static", poem::endpoint::StaticFilesEndpoint::new("./static"))
        .at("/", get(pages::hjem::hjem))
        .at("/billett", get(pages::billett::billett))
}
