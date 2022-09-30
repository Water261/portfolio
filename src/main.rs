use self::components::about::About;
use self::components::navbar::Navbar;
use self::components::projects::Projects;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <About />
            <Projects />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
