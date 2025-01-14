use perseus::prelude::*;
use sycamore::prelude::*;

fn HomePage<G: Html>(cx: Scope) -> View<G> {
    let dropdown_visible = create_signal(cx, false); //Create signals for every interactive component
    
    let toggle_dropdown = move |_| {
        dropdown_visible.set(!*dropdown_visible.get());
    };

    let handle_logout = |_| {
        // Add your logout logic here
        println!("Logging out..."); //Wipe cookies and go to login
    };

    view! { cx,
        // Navigation bar
        nav(class="bg-slate-800 text-white shadow-lg") {
            div(class="max-w-7xl mx-auto px-4") {
                div(class="flex items-center justify-between h-16") {
                    // Logo/Brand
                    div(class="flex-shrink-0 font-bold text-xl") {
                        "AppName"
                    }
                    
                    // Right side - Settings & Logout
                    div(class="flex items-center space-x-4") {
                        // Settings Dropdown
                        div(class="relative") {
                            button(
                                class="flex items-center px-3 py-2 rounded-md hover:bg-slate-700",
                                on:click=toggle_dropdown
                            ) {
                                span(class="w-5 h-5 mr-1") { "⚙️" }
                                span(class="hidden md:inline") { "Settings" }
                                span(class="w-4 h-4 ml-1") { "▼" }
                            }

                            (if *dropdown_visible.get() {
                                view! { cx,
                                    div(class="absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5") {
                                        div(class="py-1") {
                                            button(class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 w-full text-left") {
                                                "Profile Settings"
                                            }
                                            button(class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 w-full text-left") {
                                                "Account Settings"
                                            }
                                            button(class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 w-full text-left") {
                                                "Preferences"
                                            }
                                        }
                                    }
                                }
                            } else {
                                view! { cx, }
                            })
                        }

                        // Logout Button
                        button(
                            class="flex items-center px-3 py-2 rounded-md hover:bg-slate-700",
                            on:click=handle_logout
                        ) {
                            span(class="w-5 h-5 mr-1") { "🚪" }
                            span(class="hidden md:inline") { "Logout" }
                        }
                    }
                }
            }
        }

        // Page content goes here
        main {
            h1 { "Welcome to your app!" }
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
    Template::build("homepage")
        .view(|cx| {
            view! { cx,
               HomePage{}
            }
        })
        .head(head)
        .build()
}