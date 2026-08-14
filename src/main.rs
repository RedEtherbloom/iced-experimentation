use iced::{
    Element, Subscription,
    widget::{column, text},
};

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default, Debug, Clone)]
struct RssView {}

impl RssView {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("This is the ControlBar placeholder"),
            text("This is the RssScrollbar placeholder")
        ]
        .into()
    }

    #[allow(dead_code)]
    fn subscription(&self) -> Subscription<Message> {
        todo!("Placeholder for subscription logic")
    }
}

#[allow(dead_code)]
struct ControlBar {}

#[allow(dead_code)]
struct RssScrollbar {}

fn main() -> Result<(), iced::Error> {
    println!("Hello, world!");

    tracing_subscriber::fmt::init();
    iced::application(RssView::new, RssView::update, RssView::view)
        // .subscription(RssView::subscription)
        .run()
}
