use yew::prelude::*;
use yew::services::render::RenderTask;
use yew::services::RenderService;

use super::grid::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    New,
    Static,
    Merged,
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Tile {
    pub number: i32,
    pub state: TileState,
    pub previous_position: Option<Position>,
}

impl TileState {
    fn to_string(&self) -> &str {
        match self {
            TileState::New => "new",
            TileState::Static => "static",
            TileState::Merged => "merged",
        }
    }
}
impl Tile {
    pub fn new(number: i32) -> Tile {
        Tile {
            number,
            state: TileState::New,
            previous_position: None,
        }
    }
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[derive(Properties, Clone, Copy)]
pub struct TileComponentProps {
    pub tile: Tile,
    pub position: Position,
}

pub struct TileComponent {
    tile: Tile,
    position: Position,
    #[allow(dead_code)]
    timeout_task: Option<RenderTask>,
}
impl TileComponent {
    fn class_name(&self) -> String {
        format!(
            "tile tile-{} tile-{}-{} tile-{}",
            if self.tile.number <= 2048 {
                self.tile.number.to_string()
            } else {
                "super".to_string()
            },
            self.position.index() % 4,
            self.position.index() / 4,
            self.tile.state.to_string()
        )
    }
}

pub enum TileMsg {
    ActualPosition(Position),
}
impl Component for TileComponent {
    type Message = TileMsg;
    type Properties = TileComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut position = props.position;
        let mut timeout_task = None;
        match (props.tile.state, props.tile.previous_position) {
            (TileState::Merged, _) => {}
            (_, Some(previous_position)) => {
                position = previous_position;
                let actual_position = props.position;

                timeout_task = Some(RenderService::request_animation_frame(
                    link.callback(move |_| TileMsg::ActualPosition(actual_position)),
                ));
            }
            _ => {}
        }
        Self {
            timeout_task,
            tile: props.tile,
            position,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TileMsg::ActualPosition(position) => self.position = position,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.tile = _props.tile;
        self.position = _props.position;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.class_name()>
                <div class="tile-inner">
                    { self.tile.number.to_string() }
                </div>
            </div>

        }
    }
}
