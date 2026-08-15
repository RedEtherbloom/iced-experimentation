use iced::{
    Alignment, Element, Fill, Subscription, border,
    widget::{column, container, scrollable, space, text, theme},
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
            container(
                scrollable(column![
                    "Test Entry 1",
                    "Test Entry 2",
                    space().height(400),
                    "Test Entry 3",
                ])
                .height(Fill)
                .width(Fill)
            )
            .align_x(Alignment::End)
            .style(|theme: &theme::Theme| container::Style {
                border: border::Border {
                    width: 1.0,
                    radius: 5.0.into(),
                    color: theme.extended_palette().background.weak.color,
                },
                ..container::Style::default()
            })
        ]
        .align_x(Alignment::Center)
        .width(Fill)
        .padding(8)
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
