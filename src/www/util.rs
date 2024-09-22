use maud::Render;

pub struct MaudTemplate {
    title: String,
    styles: Vec<String>,
    scripts: Vec<String>,
    body: maud::Markup,
}

impl MaudTemplate {
    pub fn new(body: maud::Markup, title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            styles: Vec::new(),
            scripts: Vec::new(),
            body,
        }
    }
    pub fn with_stylesheet(mut self, stylesheet: impl ToString) -> Self {
        self.styles.push(stylesheet.to_string());
        self
    }
    pub fn with_script(mut self, script: impl ToString) -> Self {
        self.scripts.push(script.to_string());
        self
    }
    pub fn into_poem_html(self) -> poem::web::Html<maud::Markup> {
        poem::web::Html(self.render())
    }
    fn head(&self) -> maud::Markup {
        maud::html! {
            @for stylesheet in &self.styles {
                link rel="stylesheet" href=(stylesheet);
            }
            @for script in &self.scripts {
                script src=(script) type="module" {}
            }
            link rel="stylesheet" href="/static/styles.css";
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
        maud::html! {
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
        MaudTemplate {
            title: title.to_string(),
            styles: Vec::new(),
            scripts: Vec::new(),
            body: self,
        }
    }
}

impl From<MaudTemplate> for poem::web::Html<maud::Markup> {
    fn from(value: MaudTemplate) -> Self {
        value.into_poem_html()
    }
}
