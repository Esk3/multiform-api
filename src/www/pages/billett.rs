use poem::handler;

use crate::www::util::WithTemplate;

#[handler]
pub fn billett() -> poem::web::Html<maud::Markup> {
    maud::html! {
        h1 { "Billett" }
        button #my-button {
            "here is my button"
        }
        form {
            "Kjøp billett"
            input type="submit" value="her";
            div .v {
                div {
                    label for="v1" { "v1" }
                    input type="radio" name="fra" value="v1" id="v1";
                }
                div {
                    label for="v2" { "v2" }
                    input type="radio" name="fra" value="v2" id="v2";
                }
                div {
                    label for="v3" { "v3" }
                    input type="radio" name="fra" value="v3" id="v3";
                }
                div {
                    label for="v4" { "v4" }
                    input type="radio" name="fra" value="v4" id="v4";
                }
            }
        }
    }
    .with_template("Billett")
    .with_script("/static/billett.js")
    .into()
}
