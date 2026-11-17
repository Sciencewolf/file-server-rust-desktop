use crate::state::AppState;
use crate::util::level;
use dioxus::prelude::*;

#[component]
pub fn SystemPanel() -> Element {
    let state = use_context::<AppState>();
    let sys = state.sys.read();

    let Some(s) = sys.as_ref() else {
        return rsx! {
            section { class: "card",
                h2 { "System" }
                p { class: "muted", "Loading system info..." }
            }
        };
    };

    let total = s.ram_total_mb.max(1);
    let used = s.used_mb();
    let pct = used as f64 / total as f64 * 100.0;
    let ram_level = level(pct, 60.0, 85.0);
    let temp = s.metrics.cpu.temp_c;
    let temp_level = level(temp as f64, 60.0, 75.0);

    rsx! {
        section { class: "card",
            h2 { "System" }
            div { class: "sys-grid",
                span { class: "label", "Model" }
                span { class: "strong", "{s.model}" }
                span { class: "label", "CPU" }
                span { "{s.cpu}" }
                span { class: "label", "OS" }
                span { "{s.os}" }
                span { class: "label", "Temp" }
                span { class: "level-{temp_level}", "{temp:.1} °C" }
                span { class: "label", "RAM" }
                div { class: "ram",
                    div { class: "bar",
                        div { class: "fill {ram_level}", style: "width: {pct}%" }
                    }
                    span { class: "level-{ram_level}", "{used} / {total} MB ({pct:.0}%)" }
                }
                span { class: "label", "Uptime" }
                span { "{s.uptime}" }
            }
        }
    }
}
