use crate::state::{AppState, Status};
use crate::util::human_size;
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn StatusBar() -> Element {
    let state = use_context::<AppState>();
    let mut tick = use_signal(|| 0u64);

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if state.is_busy() {
                tick += 1;
            }
        }
    });

    tick.read();
    let status = state.status.read();

    match &*status {
        Status::Idle => rsx! {
            footer { class: "status muted", "Ready" }
        },
        Status::Done(m) => rsx! {
            footer { class: "status done", "✓ {m}" }
        },
        Status::Failed(m) => rsx! {
            footer { class: "status failed", "✗ {m}" }
        },
        Status::Busy(m, progress) => {
            let fraction = progress.as_ref().and_then(|p| p.fraction());
            rsx! {
                footer { class: "status busy",
                    span { class: "message", "{m}" }
                    div { class: "bar",
                        if let Some(f) = fraction {
                            div { class: "fill", style: "width: {f * 100.0}%" }
                        } else {
                            div { class: "fill indeterminate" }
                        }
                    }
                    if let (Some(f), Some(p)) = (fraction, progress.as_ref()) {
                        span { class: "muted",
                            "{f * 100.0:.0}%  {human_size(p.done())} / {human_size(p.total())}"
                        }
                    }
                }
            }
        }
    }
}
