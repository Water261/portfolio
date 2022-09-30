use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! {
		<nav class={"navbar is-fixed-top px-4 is-primary"} role={"navigation"} aria-label={"main navigation"}>
			<div class={"navbar-brand"}>
				<a href={"#"} class={"navbar-item"}>
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
	}
}