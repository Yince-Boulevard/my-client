use dioxus::prelude::*;

use crate::{api::get_top_stories, ui::story_item::Story_item};
mod story_comment;
mod story_item;
#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! {
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
            section { class: "flex flex-col pt-3 w-4/12 bg-gray-50 h-full overflow-y-scroll",
                Stories {}
            }
            section { class: "w-8/12 px-4 flex flex-col bg-white rounded-r-3xl",
                section {
                    ul { class: "mt-6", "Hello World" }
                }
            }
        }
    }
}
#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(7));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => {
            rsx! {
                ul {
                    for story in stories {
                        Story_item { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! {
                p { "Error: Failed to load stories" }
            }
        }
        None => {
            rsx! {
                p { "Loading stories..." }
            }
        }
    }
}
