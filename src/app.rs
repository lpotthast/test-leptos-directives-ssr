use leptos::{html::AnyElement, *};
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="description" content="Directives - SSR"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Stylesheet id="leptos" href=format!("/pkg/directives-ssr.css")/>
        <Title text="Directives - SSR"/>

        <Foo />
    }
}

#[derive(Clone)]
struct MyStruct {}

fn mydirective(el: HtmlElement<AnyElement>, _v: MyStruct) {
    tracing::info!("ok");
    let _el = el.attr("data-foo", "foooo");
}

#[component]
pub fn Foo() -> impl IntoView {
    view! {
        <div
            use:mydirective=MyStruct {}
        >
            "foo"
        </div>
    }
}
