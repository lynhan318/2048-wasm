use crate::components::{banner::Banner, grid::GridComponent};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct RootPage {}
impl Component for RootPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <GridComponent />
        }
    }
}
