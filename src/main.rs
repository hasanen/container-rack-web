use leptos::*;
use smartstore_box_organizer_generator::generate_svg;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (svg, set_svg) = create_signal("".to_string());

    view! {
        <button
            on:click=move |_| {
                set_svg.update(|n| n.push_str(&generate_svg(2,2, 4.0, "blue", "black").to_string()));
            }
        >
            "Generate SVG!"
        </button>
        <div id="output" inner_html={svg}>
            </div>
    }
}
