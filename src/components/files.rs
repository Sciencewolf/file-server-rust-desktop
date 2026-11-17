use crate::state::AppState;
use crate::util::{ext_color, human_size};
use dioxus::prelude::*;

#[component]
pub fn FileList() -> Element {
    let state = use_context::<AppState>();
    let files = state.files.read();
    let load_error = state.load_error.read();

    rsx! {
        section { class: "card files",
            h2 {
                "Files"
                span { class: "count", "{files.len()}" }
            }
            if let Some(err) = load_error.as_ref() {
                p { class: "error", "Could not load files: {err}" }
            }
            if files.is_empty() {
                p { class: "muted", "No files" }
            } else {
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { class: "size", "Size" }
                            th {}
                        }
                    }
                    tbody {
                        for f in files.iter() {
                            FileRow { key: "{f.name}", name: f.name.clone(), size: f.size }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FileRow(name: ReadSignal<String>, size: u64) -> Element {
    let state = use_context::<AppState>();
    let mut editing = use_signal(|| None::<String>);
    let mut confirm = use_signal(|| false);
    let busy = state.is_busy();

    let mut save = move || {
        if let Some(new) = editing() {
            state.rename(name(), new);
        }
        editing.set(None);
    };

    rsx! {
        tr {
            td { class: "name",
                if let Some(value) = editing() {
                    input {
                        class: "rename",
                        value: "{value}",
                        onmounted: move |e| async move {
                            let _ = e.set_focus(true).await;
                        },
                        oninput: move |e| editing.set(Some(e.value())),
                        onkeydown: move |e| match e.key() {
                            Key::Enter => save(),
                            Key::Escape => editing.set(None),
                            _ => {}
                        },
                    }
                } else {
                    span { style: "color: {ext_color(&name.read())}", "{name}" }
                }
            }
            td { class: "size", "{human_size(size)}" }
            td { class: "actions",
                if editing().is_some() {
                    button { class: "btn small primary", onclick: move |_| save(), "Save" }
                    button { class: "btn small", onclick: move |_| editing.set(None), "Cancel" }
                } else if confirm() {
                    span { class: "confirm", "Delete?" }
                    button {
                        class: "btn small danger",
                        onclick: move |_| {
                            confirm.set(false);
                            state.delete(name());
                        },
                        "Yes"
                    }
                    button { class: "btn small", onclick: move |_| confirm.set(false), "No" }
                } else {
                    button {
                        class: "btn small",
                        disabled: busy,
                        onclick: move |_| state.download(name()),
                        "Download"
                    }
                    button {
                        class: "btn small",
                        disabled: busy,
                        onclick: move |_| editing.set(Some(name())),
                        "Rename"
                    }
                    button {
                        class: "btn small danger",
                        disabled: busy,
                        onclick: move |_| confirm.set(true),
                        "Delete"
                    }
                }
            }
        }
    }
}
