use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BannerProps {
    pub value: i64,
}

pub struct Banner {
    link: ComponentLink<Self>,
    props: BannerProps,
}
impl Component for Banner {
    type Message = ();
    type Properties = BannerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex fc">
                <img alt="game banner" src="/game_banner.png"/>
                <span>{self.props.value}</span>
            </div>
        }
    }
}
