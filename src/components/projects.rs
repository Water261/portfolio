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
                <div class={"tile is-parent is-4"}>
                    <Project
                        project_name={"ruhroh"}
                        project_repo={"https://github.com/Water261/ruhroh"}
                        project_desc={"A logitech device manager"}
                        project_languages={vec![String::from("Rust")]}
                    />
                </div>
                <div class={"tile is-parent is-4"}>
                	<Project
                        project_name={"portfolio"}
                        project_repo={"https://github.com/Water261/portfolio"}
                        project_desc={"My portfolio website"}
                        project_languages={vec![String::from("Rust"), String::from("HTML")]}
                    />
				</div>
                <div class={"tile is-parent is-4"}>
                	<div class={"tile box is-child"}>
						<h1 class={"title"}>
							{"Three"}
						</h1>
					</div>
				</div>
            </div>
        </section>
    }
}
