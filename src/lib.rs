use std::collections::HashMap;

use container_rack_lib::{generate_svg, supported_containers};
use leptos::*;
use leptos_router::*;

const DEFAULT_COLUMNS: usize = 2;
const DEFAULT_ROWS: usize = 8;
const DEFAULT_MATERIAL: f32 = 4.0;

const COLOR_PRIMARY: &str = "blue";
const COLOR_SECONDARY: &str = "black";

#[derive(Params, PartialEq, Clone, Debug)]
struct QueryParam {
    rows: Option<usize>,
    columns: Option<usize>,
}

struct OrganizerInputs {
    rows: usize,
    columns: usize,
    material_thickness: f32,
    container: String,
}
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="navbar navbar-dark bg-dark">
                <div class="container">
                    <a class="navbar-brand">Container rack</a>

                </div>
            </nav>

            <div class="container mt-5">
                <Routes>
                    // Use `/*any` until there is another pages and/or domain has been configured with GH pages
                    <Route path="/*any" view=Generator/>
                </Routes>
                <Footer />
            </div>
        </Router>
    }
}

#[component]
fn Generator() -> impl IntoView {
    let (svg, set_svg) = create_signal("".to_string());
    let (filename, set_filename) = create_signal("".to_string());
    let navigate = leptos_router::use_navigate();
    let query = use_query_map().get_untracked();
    let containers = supported_containers();
    let container_map: HashMap<String, String> = containers
        .iter()
        .map(|c| (c.key(), format!("{}: {}", c.vendor, c.model)))
        .collect();
    let container = containers.into_iter().next().unwrap();

    let rows = query
        .get("rows")
        .unwrap_or(&DEFAULT_ROWS.to_string())
        .parse::<usize>()
        .unwrap_or(DEFAULT_ROWS);

    let columns = query
        .get("columns")
        .unwrap_or(&DEFAULT_COLUMNS.to_string())
        .parse::<usize>()
        .unwrap_or(DEFAULT_COLUMNS);

    let material = query
        .get("material")
        .unwrap_or(&DEFAULT_MATERIAL.to_string())
        .parse::<f32>()
        .unwrap_or(DEFAULT_MATERIAL);

    if query.get("rows").is_some()
        || query.get("columns").is_some()
        || query.get("material").is_some()
    {
        let generated_svg = generate_svg(
            rows,
            columns,
            material,
            &container,
            COLOR_PRIMARY,
            COLOR_SECONDARY,
        )
        .to_string();
        set_svg.update(|n| n.push_str(&generated_svg.to_string()));
    }

    let router = use_router();
    let current_path = router.pathname().get_untracked();

    view! {
            <div class="row">
                <div class="col-3">
                    <OrganizerInputsForm default_rows=rows default_columns=columns default_material_thickness=material default_container=container.key().clone() containers=container_map
                    on_submit_callback=move |inputs: OrganizerInputs| {
                        let generated_svg = generate_svg(inputs.rows,inputs.columns, inputs.material_thickness, &container, COLOR_PRIMARY, COLOR_SECONDARY).to_string();
                        set_svg.update(|n| n.clear());
                        set_svg.update(|n| n.push_str(&generated_svg.to_string()));
                        set_filename.update(|n| n.clear());
                        set_filename.update(|n| n.push_str(&format!("box_organizer_{}x{}_{}mm.svg", inputs.rows, inputs.columns, inputs.material_thickness)));
                        let new_query_params = format!("{}?rows={}&columns={}&material={}&container={}", current_path, inputs.rows, inputs.columns, inputs.material_thickness, inputs.container);
                        log::info!("new_query_params: {}", new_query_params);
                        navigate(&new_query_params, NavigateOptions::default());
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
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div class="footer d-flex justify-content-between py-4 my-4 border-top">
            <p class="text-muted"> {"Created and maintained by Joni Hasanen"}.</p>
            <ul class="list-unstyled d-flex">
                <li class="ms-3"><a class="link-dark" href="https://github.com/hasanen/container-rack-web"><img src="images/github-mark.svg" class="icon" title="GitHub" alt="GitHub"/>Web</a></li>
                <li class="ms-3"><a class="link-dark" href="https://github.com/hasanen/container-rack-lib"><img src="images/github-mark.svg" class="icon" title="GitHub" alt="GitHub"/>Lib</a></li>
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
    default_container: String,
    containers: HashMap<String, String>,
    #[prop(into)] on_submit_callback: Callback<OrganizerInputs>,
) -> impl IntoView {
    let (rows, _set_rows) = create_signal(default_rows);
    let (columns, _set_columns) = create_signal(default_columns);
    let (material_thickness, _set_material_thickness) = create_signal(default_material_thickness);
    let (selected_container_key, _set_selected_container_key) = create_signal(default_container);

    let input_rows: NodeRef<html::Input> = create_node_ref();
    let input_columns: NodeRef<html::Input> = create_node_ref();
    let input_material_thickness: NodeRef<html::Input> = create_node_ref();
    let input_container_key: NodeRef<html::Select> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value_rows = parse_value::<usize>(&input_rows, default_columns);

        let value_columns = parse_value::<usize>(&input_columns, default_columns);

        let value_material_thickness =
            parse_value::<f32>(&input_material_thickness, default_material_thickness);

        let value_container_key = input_container_key
            .get_untracked()
            .expect("<input> should be mounted")
            .value();

        on_submit_callback.call(OrganizerInputs {
            rows: value_rows,
            columns: value_columns,
            material_thickness: value_material_thickness,
            container: value_container_key,
        });
    };

    view! {
        <form on:submit=on_submit> // on_submit defined below
        <div class="mb-3">
            <label for="rows" class="form-label">Rows</label>
            <input type="number"
                min=1
                class="form-control"
                value=rows
                node_ref=input_rows
            />
            </div>
            <div class="mb-3">
            <label for="columns" class="form-label">Columns</label>
            <input type="number"
                class="form-control"
                min=1
                value=columns
                node_ref=input_columns
            />
            </div>
            <div class="mb-3">
            <label for="material_thickness" class="form-label">Material Thickness</label>
            <input type="number"
                class="form-control"
                min=1
                step=0.1
                value=material_thickness
                node_ref=input_material_thickness
            />
            </div>

            <div class="mb-3">
                <label for="container_key" class="form-label">Container</label>

                <select node_ref=input_container_key on:change=|value| {
                    log::info!("Selected container: {:?}", value);
                }>
                        {
                            containers.into_iter().map(|(key, value)| {
                                view! {
                                    <option
                                        value=&key
                                        selected=key == selected_container_key.get_untracked()
                                    >{value}</option>
                                }

                    }).collect::<Vec<_>>()
                }

                </select>
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
