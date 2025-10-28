#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::StoryItem;

#[component]
pub fn Story_item(story: StoryItem) -> Element {
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
            a { class: "flex justify-between items-center", href: "#",
                h3 { class: "text-lg font-semibold", "{story.title}" }
                p { class: "text-md text-gray-400" }
            }
            div { class: "text-md italic text-gray-400",
                span { "{story.score} points by {story.by} {story.time} | " }
                a { href: "#", "{story.kids.len()} comments" }
            }
        }
    }
}
