use sycamore::prelude::*;
fn main() {
    sycamore::render(|cx| view! { cx, 
        p { "Hello, World!" }
        p { "test "}
        ul { 
            li { "item 1"}
            li { "item 2"}
        }
    });
}
