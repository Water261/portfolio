use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! {
		<nav class={"navbar is-fixed-top px-4 is-primary"} role={"navigation"} aria-label={"main navigation"}>
			<div class={"navbar-brand"}>
				<a href={"#"} class={"navbar-item"}>
					<span class="material-symbols-outlined">{"home"}</span>
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