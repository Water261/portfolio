use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id={"about"} class={"section is-medium"}>
            <div class={"columns"}>
                <div class={"column is-flex is-flex-direction-column is-justify-content-center"}>
					<h1 class={"title"}>
						{"About Me"}
					</h1>
					<h2 class={"subtitle"}>
						{"Hi! I'm an aspiring programmer who uses Rust for their projects."}
					</h2>
                </div>
                <div class={"column is-flex is-justify-content-center is-align-items-center"}>
					<img class={"image"} src={"https://via.placeholder.com/512.png"} />
                </div>
            </div>
        </section>
    }
}
