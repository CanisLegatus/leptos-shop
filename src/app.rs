

use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style.css"/>
        <Title text="Leptos Shop"/>

        <Router>
            <header>
                <h1>"Leptos Shop"</h1>
                <nav>
                    <A href="/">Главная</A>
                    <A href="/product/1">Товар</A>
                </nav>
            </header>


            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/product/:id" view=Product/>
                </Routes>
            </main>

            <footer>Leptos Shop</footer>       
        </Router>


    }
}
