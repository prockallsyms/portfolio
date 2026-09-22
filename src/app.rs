use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::routes::{AppRoute, switch};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<AppRoute> render={Callback::from(switch)} />
        </BrowserRouter>
    }
}
