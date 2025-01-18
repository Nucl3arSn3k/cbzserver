use perseus::prelude::*;
use sycamore::prelude::*;
use stylist::style;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone)]
pub struct ThemeContext {
    theme: RcSignal<Theme>,
}

#[component]
pub fn Layout<G: Html>(cx: Scope, props: LayoutProps<G>) -> View<G> {
    let theme_signal = create_rc_signal(Theme::Dark);
    
    // Create theme context with cloned signal
    let theme_context = ThemeContext {
        theme: theme_signal.clone(),
    };
    provide_context(cx, theme_context);

    // Create theme-dependent styles with cloned signal
    let theme_signal_for_styles = theme_signal.clone();
    let theme_styles = create_memo(cx, move || {
        match *theme_signal_for_styles.get() {
            Theme::Dark => style!(
                r#"
                :root {
                    --bg-primary: #000;
                    --bg-secondary: #2a2a2a;
                    --text-primary: #fff;
                    --text-secondary: #ccc;
                    --accent: #333;
                    --hover: #444;
                }
                "#
            ).unwrap(),
            Theme::Light => style!(
                r#"
                :root {
                    --bg-primary: #fff;
                    --bg-secondary: #f5f5f5;
                    --text-primary: #000;
                    --text-secondary: #333;
                    --accent: #e0e0e0;
                    --hover: #d0d0d0;
                }
                "#
            ).unwrap(),
        }
    });

    let base_styles = style!(
        r#"
        .topnav {
            background-color: var(--bg-primary);
            padding: 1rem;
            display: flex;
            gap: 1rem;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        
        .topnav a {
            color: var(--text-primary);
            text-decoration: none;
            padding: 0.5rem 1rem;
            border-radius: 4px;
            transition: background-color 0.2s;
        }
        
        .topnav a:hover {
            background-color: var(--accent);
        }
        
        .tabledisplay {
            padding: 2rem;
            background-color: var(--bg-secondary);
            min-height: calc(100vh - 60px);
            color: var(--text-primary);
        }
        
        main {
            max-width: 38em;
            margin: 0 auto;
            background-color: var(--bg-primary);
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }
        
        .theme-toggle {
            margin-left: auto;
            background: var(--accent);
            border: none;
            color: var(--text-primary);
            padding: 0.5rem 1rem;
            border-radius: 4px;
            cursor: pointer;
        }
        
        .theme-toggle:hover {
            background: var(--hover);
        }
        "#
    ).unwrap();

    // Create separate signals for the toggle and view
    let theme_signal_for_toggle = theme_signal.clone();
    let theme_signal_for_view = theme_signal.clone();

    let toggle_theme = move |_| {
        let current = (*theme_signal_for_toggle.get()).clone();
        theme_signal_for_toggle.set(match current {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        });
    };

    view! { cx,
        div(class=theme_styles.get().get_class_name()) {
            div(class=base_styles.get_class_name()) {
                nav(class="topnav") {
                    a(href="/") { "Home" }
                    a(href="/settings") { "Settings" }
                    a(href="/login") { "Logout" }
                    button(
                        class="theme-toggle",
                        on:click=toggle_theme
                    ) {
                        (match *theme_signal_for_view.get() {
                            Theme::Dark => "Switch to Light Theme",
                            Theme::Light => "Switch to Dark Theme",
                        })
                    }
                }
                div(class="tabledisplay") {
                    main {
                        h1 { (props.title) }
                        (props.children.call(cx))
                    }
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub title: &'static str,
    pub children: Children<'a, G>,
}