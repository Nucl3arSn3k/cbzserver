use perseus::prelude::*;
use sycamore::prelude::*;

fn HomePage<G: Html>(cx: Scope) -> View<G> { //Ignore rust warnings,this needs to be like this due to elm
    let dropdown_visible = create_signal(cx, false); //Create signals for every interactive component
    
    let toggle_dropdown = move |ev: web_sys::Event| {
        dropdown_visible.set(!*dropdown_visible.get());
    };

    let handle_logout = |_: web_sys::Event| {
        // Add your logout logic here
        println!("Logging out..."); //Wipe cookies and go to login
    };

    view! { cx,
        // Navigation bar
        div(class="topnav") {
            a(href="/") { "Home" }
            a(href="/settings") { "Settings" }
            a(
                href="#",
                on:click=|_| {
                    // Handle logout logic here
                    navigate("/login");
                }
            ) { "Logout" }
        }
        // Page content goes here
        div(class="tabledisplay"){
            main {
                h1 { "Welcome to your app!" }
            }


        }
        
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Login - My App" }
        link(rel="stylesheet", href="https://unpkg.com/sakura.css/css/sakura-dark.css", type="text/css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("homepage")
        .view(|cx| {
            view! { cx,
               HomePage{}
            }
        })
        .head(head)
        .build()
}