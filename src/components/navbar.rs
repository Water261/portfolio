use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! {
		<div class={"container is-fluid"}>
			<nav class={"navbar"} role={"navigation"} aria-label={"main navigation"}>
				<div class={"navbar-brand"}>
					<a class={"navbar-item"}>
						<img class={"image is-24x24"} src={"https://via.placeholder.com/24.png"} />
					</a>
				</div>

				<div class={"navbar-menu"}>
					<div class={"navbar-end"}>
						<a href={"#projects"} class={"navbar-item"}>
							{"Projects"}
						</a>
						<a href={"#about"} class={"navbar-item"}>
							{"About"}
						</a>
					</div>
				</div>
			</nav>
		</div>
	}
}