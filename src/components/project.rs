use std::ops::Index;

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
	// TODO: Make this more elegant
	let mut languages = String::new();
	let lang_len = props.project_languages.len();

	for lang in &props.project_languages {
		let index = match props.project_languages.binary_search(lang) {
			Ok(i) => i,
			Err(_) => 0,
		};

		if index == lang_len - 1 {
			languages.push_str(lang);
		} else {
			languages.push_str(
				format!("{}, ", lang).as_str()
			);
		}
	}

	html! {
		<div class={"tile box is-child"}>
			<h1 class={"title"}>
				{ &props.project_name }
			</h1>
			<a target={"_blank"} href={ format!("{}", &props.project_repo) }>
				<h3 class={"subtitle pb-3"}>
					{ &props.project_repo }
				</h3>
			</a>
			<p class={"py-3"}>{ &props.project_desc }</p>
			<p class={"py-3"}>
				{format!("Built in: {}", languages)}
			</p>
		</div>
	}
}