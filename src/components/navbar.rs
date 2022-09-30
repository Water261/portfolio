use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! {
		<nav class={"navbar"} role={"navigation"} aria-label={"main navigation"}>
			<div class={"navbar-brand"}>
				<a class={"navbar-item"}>
					{"Icon"}
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