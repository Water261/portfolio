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
						{"Hi there! I'm Matthew, better known through my online alias of Water261. I'm an aspiring programming working on my skills with Rust and C#."}
					</h2>
                </div>
                <div class={"column is-flex is-justify-content-center is-align-items-center"}>
					<img class={"image"} src={"https://via.placeholder.com/512.png"} />
                </div>
            </div>
        </section>
    }
}
