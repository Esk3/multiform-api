use poem::handler;

use crate::www::util::WithTemplate;

#[handler]
pub fn billett() -> poem::web::Html<maud::Markup> {
    maud::html! {
        h1 { "Billett" }
        form {
            "Kjøp billett"
            input type="submit" value="her";
        }
    }
    .with_template("Billett")
    .into()
}
