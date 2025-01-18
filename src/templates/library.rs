use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::layout::Layout;

#[component]
fn HomePage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title="Welcome to your app!") {
            p { "Welcome to your application! This is your homepage content." }
            p { "You can add more content here." }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "My App" }
        meta(charset="UTF-8")
        meta(name="viewport", content="width=device-width, initial-scale=1.0")
        link(
            rel="stylesheet",
            href="https://unpkg.com/sakura.css/css/sakura-dark.css",
            type="text/css"
        )
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("homepage")
        .view(|cx| {
            view! { cx,
                HomePage {}
            }
        })
        .head(head)
        .build()
}