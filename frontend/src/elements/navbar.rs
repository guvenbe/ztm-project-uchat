#![allow(non_snake_case)]

use crate::{page::POST_NEW_CHAT, prelude::*};
use dioxus::prelude::*;

#[inline_props]
pub fn NewPostPopup(cx: Scope, hide: UseState<bool>) -> Element {
    let router = use_router(cx);

    let hide_class = maybe_class!("hidden", *hide.get());

    const BUTTON_CLASS: &str = "flex flex-col items-center justify-center gap-1
                                w-full cursor-pointer hover:bg-slate-500/40 active:bg-slate-500/60 border-t first:border-t-0 navbar-border-color";
    cx.render(rsx! {
        div {
            class: "flex flex-col
                    absolute right-0 z-50 pointer-events-auto
                    items-stretch {hide_class}
                    navbar-bg-color text-white text-sm shadow-lg border navbar-border-color",
            style: "bottom: var(--navbar-height); width: calc(100% / 6);",
            div {
                class: BUTTON_CLASS,
                style: "height: var(--navbar-height);",
                onclick: move |_| (),
                img {
                    class: "invert",
                    src: "/static/icons/icon-poll.svg",
                    width: "25px",
                    height: "25px",
                },
                "Poll"
            }
            div {
                class: BUTTON_CLASS,
                style: "height: var(--navbar-height);",
                onclick: move |_| (),
                img {
                    class: "invert",
                    src: "/static/icons/icon-image.svg",
                    width: "25px",
                    height: "25px",
                },
                "Image"
            }
            div {
                class: BUTTON_CLASS,
                style: "height: var(--navbar-height);",
                onclick: move |_| {
                    router.navigate_to(POST_NEW_CHAT);
                    hide.set(true);
                },
                img {
                    class: "invert",
                    src: "/static/icons/icon-messages.svg",
                    width: "25px",
                    height: "25px",
                },
                "Chat"
            }
        }
    })
}

#[derive(Props)]
pub struct NavButtonProps<'a> {
    img: &'a str,
    label: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
    highlight: Option<bool>,
    children: Element<'a>,
}

pub fn NavButton<'a>(cx: Scope<'a, NavButtonProps<'a>>) -> Element {
    let selected_bgcolor = maybe_class!("bg-slate-500", matches!(cx.props.highlight, Some(true)));

    cx.render(rsx! {
        button {
            class: "relative cursor-pointer flex flex-col items-center justify-center h-full navbar-bg-color text-white hover:bg-slate-500/40 active:bg-slate-500/60 border-l first:border-l-0 navbar-border-color {selected_bgcolor}",
            onclick: move |ev| cx.props.onclick.call(ev),
            img {
                class: "invert",
                src: cx.props.img,
                width: "25px",
                height: "25px",
            },
            div {
                class: "text-sm text-white",
                cx.props.label
            },
            &cx.props.children
        }
    })
}

pub fn Navbar(cx: Scope) -> Element {
    let hide_new_post_popup = use_state(cx, || true);

    cx.render(rsx! {
        nav {
            class: "relative fixed bottom-0 left-0 right-0 w-full
                border-t navbar-bg-color navbar-border-color",
            style: "height: var(--navbar-height);",
            div {
                class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
                NavButton {
                    img: "/static/icons/icon-home.svg",
                    label: "Home",
                    onclick: |_| (),
                },
                NavButton {
                    img: "/static/icons/icon-trending.svg",
                    label: "Trending",
                    onclick: |_| (),
                }
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: move |_| {
                        let is_hidden = *hide_new_post_popup.get();
                        hide_new_post_popup.set(!is_hidden);
                    }
                }
            }
            NewPostPopup { hide: hide_new_post_popup.clone() }
        }
    })
}