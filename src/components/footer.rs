use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! {cx, 
        footer (class="absolute bottom-0 px-10"){
            p {"test footer"}
        }
    }
}