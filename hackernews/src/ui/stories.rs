#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::{
    api::{get_story_comments, get_top_stories},
    ui::CommentState,
    StoryData, StoryItem,
};

#[component]
pub fn StoryItemView(story: StoryItem) -> Element {
    let mut comment_state = use_context::<Signal<CommentState>>();
    let _full_story = use_signal::<Option<StoryData>>(|| None);
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
            a { class: "flex justify-between items-center", href: "#",
                h3 { class: "text-lg font-semibold", "{story.title}" }
                p { class: "text-md text-gray-400" }
            }
            div { class: "text-md italic text-gray-400",
                span { "{story.score} points by {story.by} {story.time} | " }
                a {
                    href: "#",
                    onclick: move |evt| {
                        let story = story.clone();
                        evt.prevent_default();
                        info!("Clicked on story: {}", story.title);
                        async move {
                            *comment_state.write() = CommentState::Loading;
                            if let Ok(story_data) = get_story_comments(story).await {
                                *comment_state.write() = CommentState::Loaded(story_data);
                            }
                        }
                    },
                    "{story.kids.len()} comments"
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
                        StoryItemView { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { class: "mt-6 text-red-500",
                    p { "Failed to load stories" }
                    p { "{err}" }
                }
            }
        }
        None => {
            rsx! {
                p { "Loading stories..." }
            }
        }
    }
}
