#![allow(non_snake_case)]
use crate::{ui::CommentState, Comment};
use dioxus::prelude::*;

#[component]
pub fn StoryCommentView(comment: Comment) -> Element {
    rsx! {
        li {
            article { class: "p-4 text-gray-500 leading-7 tracking-wider border-b border-gray-200",
                span { "{comment.by} {comment.time} | next [-]" }
                div { dangerous_inner_html: "{comment.text}" }
            }
        }
    }
}
#[component]
pub fn Comments() -> Element {
    let comment_state = use_context::<Signal<CommentState>>();
    let state = comment_state.read();
    match &*state {
        CommentState::Unset => {
            rsx! {
                div { class: "mt-6",
                    p { "Please Select a comment to view details" }
                }
            }
        }
        CommentState::Loading => {
            rsx! {
                div { class: "mt-6",
                    p { "Loading comments..." }
                }
            }
        }
        CommentState::Loaded(story_data) => {
            rsx! {
                ul {
                    for comment in &story_data.comments {
                        StoryCommentView { comment: comment.clone() }
                    }
                }
            }
        }
    }
}
