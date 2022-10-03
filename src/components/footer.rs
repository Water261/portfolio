use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
	html! {
		<footer class={"footer"}>
			<div class={"content has-text-centered"}>
				<p>
					<span><strong>{"Portfolio"}</strong></span>
					<span>{" by Matthew Wilks. The source code is licensed "}</span>
					<span><a target={"_blank"} href={"https://opensource.org/licenses/mit-license.php"}>{"MIT"}</a></span>
					<span>{". The website content is licensed "}</span>
					<span><a target={"_blank"} href={"https://creativecommons.org/licenses/by-nc-sa/4.0/"}>{"CC BY NC SA 4.0"}</a></span>
					<span>{"."}</span>
				</p>
			</div>
		</footer>
	}
}