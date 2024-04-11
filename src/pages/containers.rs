use container_rack_lib::supported_containers;
use leptos::*;

#[component]
pub fn SupportedContainers() -> impl IntoView {
    view! {
        <div>
          Currently supported containers.
          <table class="table containers">
            <thead>
              <tr>
                <th class="vendor">Vendor</th>
                <th class="model">Model</th>
                <th class="description">Description</th>
                <th class="links">Links</th>
              </tr>
            </thead>
            <tbody>
              {
                  supported_containers().into_iter().map(|c| {
                      view! {
                          <tr>
                            <td>{c.vendor}</td>
                            <td>{c.model}</td>
                            <td>{c.description}</td>
                            <td>
                              <ul>
                                {
                                    c.links.iter().map(|l| {
                                        view! {
                                            <li>
                                              <a href={l.url.to_string()} rel="noreferrer noopener" target="_blank">{l.title.to_string()}</a>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()
                                }
                                </ul>
                            </td>
                          </tr>
                      }
                  }).collect::<Vec<_>>()
              }
            </tbody>
          </table>
        </div>
    }
}
