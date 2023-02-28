use components::header::Header;
use components::footer::Footer;
use components::card::Card;

use sycamore::prelude::*;

mod components;



#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);
    let increment = |_| state.set(*state.get() + 1);
    let decrement = |_| state.set(*state.get() - 1);
    let reset = |_| state.set(0);
    let test_state = create_signal(cx, String::from("Test Card"));
    view! { 
        cx, 
        Header {}
        div(class="container mx-auto") {
            Card(title = test_state)

            p { "Value: " (state.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
            button(on:click=reset) { "Reset" }  
        }
        Footer {}

    }
}

fn main() {
    sycamore::render(App);
}