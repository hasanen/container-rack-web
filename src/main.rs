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

    view! {
        <div class="container">
            <nav>
                <a href="https://github.com/hasanen/smartstore-box-organizer-web"><img src="images/github-mark.svg" class="icon" title="GitHub" alt="GitHub"/></a>
            </nav>
        </div>
        <OrganizerInputsForm default_rows=8 default_columns=2 default_material_thickness=4.0
        on_submit_callback=move |inputs: OrganizerInputs| {
            let generated_svg = generate_svg(inputs.rows,inputs.columns, inputs.material_thickness, "blue", "black").to_string();
            set_svg.update(|n| n.clear());
            set_svg.update(|n| n.push_str(&generated_svg.to_string()));
        }
        />

        <div id="output" inner_html={svg}>
            </div>
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
            <label for="rows">Rows:</label>
            <input type="number"
                value=rows
                node_ref=input_rows
            />
            <label for="columns">Columns:</label>
            <input type="number"
                value=columns
                node_ref=input_columns
            />
            <label for="material_thickness">Material Thickness:</label>
            <input type="number"
                value=material_thickness
                node_ref=input_material_thickness
            />
            <input type="submit" value="Generate SVG!"/>
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
