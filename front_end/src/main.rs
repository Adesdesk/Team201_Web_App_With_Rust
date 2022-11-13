use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Hello Everyone!"}</h2>
            <h3 class={"heading"}>{"Welcome to the rust-based web application developed by Team201"}</h3>
        </div>
    }
}