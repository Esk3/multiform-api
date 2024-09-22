use poem::handler;

use crate::www::util::WithTemplate;

#[handler]
pub fn hjem() -> poem::web::Html<maud::Markup> {
    maud::html! {
        h1 {
            "Fly billetter til salgs"
        }
        p {
            "fin din drøme reise"
        }
    }
    .with_template("Hjem")
    .into()
}
