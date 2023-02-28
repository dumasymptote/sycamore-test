//use chrono::prelude::*;
use sycamore::prelude::*;


#[derive(Prop)]
pub struct CardProps<'a>{
    title: &'a ReadSignal<String>,
    //content: String,
    //entered: DateTime<Local>,
    //completed: DateTime<Local>, 
    //due: DateTime<Local>,
    //finished: bool
}

#[component]
pub fn Card<'a, G:Html> (cx: Scope<'a>, props: CardProps<'a>) -> View<G> {
    view! {cx, 
        div ( class="test" ) {
            h3 { (props.title.get()) }
        }
    }
}
