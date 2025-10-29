use dioxus::prelude::*;

use crate::{
    ui::{comments::Comments, stories::Stories},
    StoryData,
};
mod comments;
mod stories;

#[derive(Debug, Clone)]
pub enum CommentState {
    Unset,
    Loading,
    Loaded(StoryData),
}
#[allow(non_snake_case)]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentState::Unset));
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css"),
        }
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
            section { class: "flex flex-col pt-3 w-4/12 bg-gray-50 h-full overflow-y-scroll",
                Stories {}
            }
            section { class: "w-8/12 px-4 flex flex-col bg-white rounded-r-3xl", Comments {} }
        }
    }
}
