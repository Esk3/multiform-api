use poem::handler;

use crate::www::util::WithTemplate;

#[handler]
pub fn billett() -> poem::web::Html<maud::Markup> {
    maud::html! {
        h1 { "Billett" }
        form {
            (lufthavn_search_component("fra"))
            hr;
            (lufthavn_search_component("til"))
            input type="submit" value="neste";
        }
    }
    .with_template("Billett")
    .with_script("/static/billett.js")
    .into()
}

fn lufthavn_search_component(name: impl ToString) -> maud::Markup {
    maud::html! {
        div .lufthavn-search {
            input type="text";
            div .options data-name=(name.to_string()) {}
        }
    }
}
