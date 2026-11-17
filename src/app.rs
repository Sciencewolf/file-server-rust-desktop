use crate::components::{files::FileList, status::StatusBar, system::SystemPanel};
use crate::state::AppState;
use dioxus::prelude::*;
use std::time::Duration;

const STYLE: &str = include_str!("../assets/style.css");
const POLL_INTERVAL: Duration = Duration::from_secs(5);

#[component]
pub fn App() -> Element {
    let state = use_context_provider(AppState::new);

    use_future(move || async move {
        loop {
            state.refresh_sys().await;
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    });

    use_future(move || async move {
        loop {
            state.refresh_files().await;
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    });

    let busy = state.is_busy();

    rsx! {
        style { {STYLE} }
        div { class: "app",
            header { class: "header",
                h1 { "File Server" }
                div {
                    button {
                        class: "btn",
                        onclick: move |_| {
                            spawn(state.refresh_files());
                        },
                        "Refresh"
                    }
                    button {
                        class: "btn primary",
                        disabled: busy,
                        onclick: move |_| state.upload(),
                        "Upload file"
                    }
                }
            }
            SystemPanel {}
            FileList {}
            StatusBar {}
        }
    }
}
