use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
	pub project_name: String,
	pub project_repo: String,
	pub project_desc: String,
	pub project_languages: Vec<String>,
}

#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
	html! {
		<div class="tile box is-child">
			<h1 class={"title"}>
				{ &props.project_name }
			</h1>
			<a href={ format!("{}", &props.project_repo) }>
				<h3 class={"subtitle"}>
					{ &props.project_repo }
				</h3>
			</a>
			<p>{ &props.project_desc }</p>
			<ul>
				{ props.project_languages.iter().collect::<Html>() }
			</ul>
		</div>
	}
}