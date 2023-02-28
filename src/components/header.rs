use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! {cx, 
        header(class="bg-gray-800"){
            nav(class="mx-auto flex max-w-7xl items-right justify-end p-6 lg:px-8"){
                div(class="flex lg:flex lg:gap-x-12"){
                    a(class="block rounded-lg py-2 px-3 text-base font-semibold leading-7 text-gray-200 hover:bg-gray-50 hover:text-gray-800", href="#"){ "Home" }
                    a(class="block rounded-lg py-2 px-3 text-base font-semibold leading-7 text-gray-200 hover:bg-gray-50 hover:text-gray-800", href="#"){ "Tasks" }
                    a(class="block rounded-lg py-2 px-3 text-base font-semibold leading-7 text-gray-200 hover:bg-gray-50 hover:text-gray-800", href="#"){ "Calendar" }
                }
            }
        }
    }
}