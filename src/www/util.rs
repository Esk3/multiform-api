pub struct MaudTemplate {
    title: String,
    body: maud::Markup,
}

impl MaudTemplate {
    pub fn new(body: maud::Markup, title: impl ToString) -> Self {
        Self { title: title.to_string(), body }
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
