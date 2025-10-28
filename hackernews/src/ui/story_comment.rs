#![allow(non_snake_case)]
use crate::StoryItem;
use dioxus::prelude::*;

#[component]
pub fn Story_item(item: StoryItem) -> Element {
    rsx! {
        section {
            h1 { class: "font-bold text-2xl", "We need UI/UX designer" }
            article { class: "mt-8 text-gray-500 leading-7 tracking-wider",
                p { "Hi Akhil," }
                p {
                    "Design and develop enterprise-facing UI and consumer-facing UI as well as\r\n            REST API\r\n            backends.Work with\r\n            Product Managers and User Experience designers to create an appealing user experience for\r\n            desktop\r\n            web and\r\n            mobile web."
                }
                footer { class: "mt-12",
                    p { "Thanks & Regards," }
                    p { "Alexandar" }
                }
            }
        }
    }
}
