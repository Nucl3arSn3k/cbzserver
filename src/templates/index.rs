// templates/login.rs
use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
fn LoginPage<G: Html>(cx: Scope) -> View<G> {
    let username = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());
    let error = create_signal(cx, String::new());

    let handle_submit = move |ev: web_sys::Event| {
        ev.prevent_default();
        
        if username.get().is_empty() || password.get().is_empty() {
            error.set("Please fill in all fields".to_string());
            return;
        }
        
        // In a real app, you'd handle authentication here
        if *username.get() == "name" && *password.get() == "blarch" { //have to dereference to get str
            // Perseus provides utilities for client-side navigation
            
            navigate("/homepage");
        } else {
            error.set("Invalid credentials".to_string());
        }
    };

    view! { cx,
        div(class="container") {
            h1 { "Login" }
            form(on:submit=handle_submit, class="login-form") {
                div(class="form-group") {
                    label { "Username:" }
                    input(
                        type="text",
                        bind:value=username,
                        class="form-control"
                    )
                }
                div(class="form-group") {
                    label { "Password:" }
                    input(
                        type="password",
                        bind:value=password,
                        class="form-control"
                    )
                }
                (if !error.get().is_empty() {
                    view! { cx,
                        div(class="error") {
                            (error.get())
                        }
                    }
                } else {
                    view! { cx, }
                })
                button(type="submit", class="btn") {
                    "Login"
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Login - My App" }
        link(rel="stylesheet", href="/static/styles.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .view(|cx| {
            view! { cx,
                LoginPage {}
            }
        })
        .head(head)
        .build()
}