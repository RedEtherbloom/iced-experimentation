use std::{collections::HashMap, ops::Mul, sync::Weak};

use iced::{
    Pixels, Point, Rectangle, Renderer, Size, Theme, mouse, widget::{canvas::{self, Path, path::lyon_path::Position}, text_editor::Position}
};

#[derive(Debug, Clone)]
enum Message {}

#[derive(Debug, Clone)]
struct Node {
    nodes_in: Vec<Weak<Node>>,
    nodes_out: Vec<Weak<Node>>,
    id: u64,
    // Random uuid to display. Just as a test.
    random_uuid: String,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            nodes_in: Default::default(),
            nodes_out: Default::default(),
            id: Default::default(),
            random_uuid: uuid::Uuid::now_v7().into(),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct State {
    nodes: Vec<Node>,
    next_id: u64,
    settings: Settings,
}

#[derive(Default, Debug, Clone)]
struct Settings {
    node_settings: NodeSettings,
}

#[derive(Debug, Clone)]
struct NodeSettings {
    font_size: Pixels,
    padding_node: Size,
    node_distance: Size,
    node_offset: Point,
}

impl Default for NodeSettings {
    fn default() -> Self {
        Self {
            font_size: Pixels::from(12.0),
            padding_node: Size::new(10.0, 10.0),
            node_offset: Point::new(40.0, 40.0),
            node_distance: Size::new(20.0, 0.0),
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let node_settings = &self.settings.node_settings;
        let mut frame = canvas::Frame::with_bounds(renderer, bounds);

        // TODO: Offset between nodes from last element
        let mut node_paths = HashMap::<u64, &Path>::new();
        let node_offset = node_settings.node_offset;
        for (i, node) in self.nodes.iter().enumerate() {
            let text_widget = iced::widget::canvas::Text { content: node.random_uuid.clone(), size: node_settings.font_size, ..Default::default() };

            // Path::rounded_rectangle(Size::new(0.0, 20.0) + node_settings.node_distance, (80.0, 80.0).into(), 12.0.into());
        }

        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
