use chrono::Datelike;
use leptos::*;
use smartstore_box_organizer_generator::generate_svg;

fn main() {
    mount_to_body(App)
}

struct OrganizerInputs {
    rows: usize,
    columns: usize,
    material_thickness: f32,
}
#[component]
fn App() -> impl IntoView {
    let (svg, set_svg) = create_signal("".to_string());
    let (filename, set_filename) = create_signal("".to_string());

    view! {
            <nav class="navbar navbar-dark bg-dark">
            <div class="container">
                <a class="navbar-brand">Box Organizer</a>

            </div>
            </nav>

            <div class="container mt-5">
            <div class="row">
                <div class="col-3">
                    <OrganizerInputsForm default_rows=8 default_columns=2 default_material_thickness=4.0
                    on_submit_callback=move |inputs: OrganizerInputs| {
                        let generated_svg = generate_svg(inputs.rows,inputs.columns, inputs.material_thickness, "blue", "black").to_string();
                        set_svg.update(|n| n.clear());
                        set_svg.update(|n| n.push_str(&generated_svg.to_string()));
                        set_filename.update(|n| n.clear());
                        set_filename.update(|n| n.push_str(&format!("box_organizer_{}x{}_{}mm.svg", inputs.rows, inputs.columns, inputs.material_thickness)));
                    }
                    />
                    <Show
                        when=move || { svg.get().len() > 0 }
                        fallback=|| view! {}
                    >
                        <DownloadSVG svg=svg.get() filename=filename.get()/>
                    </Show>
                </div>
                <div class="col-9">
                    <div id="output" inner_html={svg}>
                    </div>
                </div>
            </div>

            <Footer />
    </div>
        }
}

#[component]
fn Footer() -> impl IntoView {
    let current_year = chrono::Utc::now().year();

    view! {
        <div class="footer d-flex justify-content-between py-4 my-4 border-top">
            <p class="text-muted"> {format!("© {} Joni Hasanen. All rights reserved", current_year)}.</p>
            <ul class="list-unstyled d-flex">
                <li class="ms-3"><a class="link-dark" href="https://github.com/hasanen/smartstore-box-organizer-web"><img src="images/github-mark.svg" class="icon" title="GitHub" alt="GitHub"/>Web</a></li>
                <li class="ms-3"><a class="link-dark" href="https://github.com/hasanen/smartstore-box-organizer-generator"><img src="images/github-mark.svg" class="icon" title="GitHub" alt="GitHub"/>Lib</a></li>
            </ul>
        </div>
    }
}

#[component]
fn DownloadSVG(svg: String, filename: String) -> impl IntoView {
    view! {
        <p class="mt-3">
        <a href={format!("data:image/svg+xml,{}", svg)} download=filename>
            <button class="btn btn-success">Download</button>
        </a>
        </p>
    }
}

#[component]
fn OrganizerInputsForm(
    default_rows: usize,
    default_columns: usize,
    default_material_thickness: f32,
    #[prop(into)] on_submit_callback: Callback<OrganizerInputs>,
) -> impl IntoView {
    let (rows, _set_rows) = create_signal(default_rows);
    let (columns, _set_columns) = create_signal(default_columns);
    let (material_thickness, _set_material_thickness) = create_signal(default_material_thickness);

    let input_rows: NodeRef<html::Input> = create_node_ref();
    let input_columns: NodeRef<html::Input> = create_node_ref();
    let input_material_thickness: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value_rows = parse_value::<usize>(&input_rows, default_columns);

        let value_columns = parse_value::<usize>(&input_columns, default_columns);

        let value_material_thickness =
            parse_value::<f32>(&input_material_thickness, default_material_thickness);

        on_submit_callback.call(OrganizerInputs {
            rows: value_rows,
            columns: value_columns,
            material_thickness: value_material_thickness,
        });
    };

    view! {
        <form on:submit=on_submit> // on_submit defined below
        <div class="mb-3">
            <label for="rows" class="form-label">Rows</label>
            <input type="number"
                class="form-control"
                value=rows
                node_ref=input_rows
            />
            </div>
            <div class="mb-3">
            <label for="columns" class="form-label">Columns</label>
            <input type="number"
            class="form-control"
                value=columns
                node_ref=input_columns
            />
            </div>
            <div class="mb-3">
            <label for="material_thickness" class="form-label">Material Thickness</label>
            <input type="number"
                class="form-control"
                value=material_thickness
                node_ref=input_material_thickness
            />
            </div>
            <button type="submit" class="btn btn-primary">Generate</button>
        </form>
    }
}

fn parse_value<T: std::str::FromStr>(input: &NodeRef<html::Input>, default: T) -> T {
    input
        .get_untracked()
        .expect("<input> should be mounted")
        .value()
        .parse::<T>()
        .unwrap_or(default)
}
