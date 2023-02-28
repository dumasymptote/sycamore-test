use components::header::Header;
use components::footer::Footer;
use sycamore::prelude::*;

mod components;



#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);
    let increment = |_| state.set(*state.get() + 1);
    let decrement = |_| state.set(*state.get() - 1);
    let reset = |_| state.set(0);
    view! { cx,
        
        div(class="container mx-auto") {
            Header {}
            p { "Value: " (state.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
            button(on:click=reset) { "Reset" }
            Footer {}
        }

    }
}

fn main() {
    sycamore::render(App);
}