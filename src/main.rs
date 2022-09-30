use self::components::about::About;
use self::components::navbar::Navbar;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <About />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
