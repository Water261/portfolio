use yew::prelude::*;
use self::components::navbar::Navbar;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Navbar />
    }
}

fn main() {
    yew::start_app::<App>();
}
