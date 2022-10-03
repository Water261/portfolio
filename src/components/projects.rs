use yew::prelude::*;
use crate::components::project::Project;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id={"projects"} class={"section is-medium"}>
            <h1 class={"title"}>
                {"Projects"}
            </h1>
            <div class={"tile is-ancestor"}>
                <div class={"tile is-parent is-6"}>
                    <Project
                        project_name={"ruhroh"}
                        project_repo={"https://github.com/Water261/ruhroh"}
                        project_desc={"Built in Rust, ruhroh aims to be a complete replica of PixlOne's logiops project. Ruhroh is designed to handle Logitech devices through its simple configuration file written in JSON."}
                        project_languages={vec![String::from("Rust")]}
                    />
                </div>
                <div class={"tile is-parent is-6"}>
                	<Project
                        project_name={"portfolio"}
                        project_repo={"https://github.com/Water261/portfolio"}
                        project_desc={"Built with yew, portfolio is designed to serve as my personal portfolio while demonstrating the power of WebAssembly. It is designed to be easy to modify while still looking professional, portfolio achieves this through the use of the Bulma CSS framework."}
                        project_languages={vec![String::from("Rust"), String::from("Yew")]}
                    />
				</div>
            </div>
        </section>
    }
}
