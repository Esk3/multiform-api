use maud::Render;

pub struct MaudTemplate {
    title: String,
    body: maud::Markup,
}

impl MaudTemplate {
    pub fn new(body: maud::Markup, title: impl ToString) -> Self {
        Self { title: title.to_string(), body }
    }
    pub fn into_poem_html(self) -> poem::web::Html<maud::Markup> {
        poem::web::Html(self.render())
    }
    fn head(&self) -> maud::Markup {
        maud::html! {
            title { (self.title) }
        }
    }
    fn header() -> maud::Markup {
        maud::html! {
            header {
                (Self::navbar())
            }
        }
    }
    fn navbar() -> maud::Markup {
        maud::html!{
            nav {
                ul {
                    li { a href="/" {"home"}}
                }
            }
        }
    }
    fn footer() -> maud::Markup {
        maud::html! {
            footer {
                "some footer"
            }
        }
    }
}

impl maud::Render for MaudTemplate {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (maud::DOCTYPE)
            html {
                (self.head())
                body {
                    (Self::header())
                    main {
                        (self.body)
                    }
                    (Self::footer())
                }
            }
        }
    }
}

pub trait WithTemplate {
    fn with_template(self, title: impl ToString) -> MaudTemplate;
}

impl WithTemplate for maud::Markup {
    fn with_template(self, title: impl ToString) -> MaudTemplate {
        MaudTemplate { title: title.to_string(), body: self }
    }
}

impl From<MaudTemplate> for poem::web::Html<maud::Markup> {
    fn from(value: MaudTemplate) -> Self {
        value.into_poem_html()
    }
}
