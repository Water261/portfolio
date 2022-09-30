use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id={"projects"} class={"section is-medium"}>
            <h1 class={"title"}>
                {"Projects"}
            </h1>
            <div class={"tile is-ancestor"}>
                <div class={"tile is-parent is-4"}>
                    <div class={"tile box is-child"}>
                        <h1 class={"title"}>
                            {"One"}
                        </h1>
                    </div>
                </div>
                <div class={"tile is-parent is-4"}>
                	<div class={"tile box is-child"}>
						<h1 class={"title"}>
							{"Two"}
						</h1>
					</div>
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
