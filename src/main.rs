use iced::{
    Alignment, Element, Fill, Font, Pixels, Subscription,
    advanced::text::Wrapping,
    border,
    font::Weight,
    widget::{
        button, column, container, keyed_column, row, scrollable, text, text::LineHeight, theme,
    },
};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    TrashEntry(Uuid),
    SaveEntry(Uuid),
}

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
        const DEFAULT_ICED_FONT_SIZE: f32 = 16.0;
        let height_single_line = LineHeight::default().to_absolute(Pixels(DEFAULT_ICED_FONT_SIZE));
        column![
            text("This is the ControlBar placeholder"),
            container(
                scrollable(
                    keyed_column((1..=100).map(|i| { let uuid = uuid::Uuid::now_v7();
                        (
                            uuid,
                            container((row![
                                column![
                                    container(text!("Element number {i}").font(Font {
                                        weight: Weight::Semibold,
                                        ..Default::default()
                                    })).id(format!("Rss-Item-{i}")),
                                    text!("This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder!")
                                        .font(Font {
                                            weight: Weight::ExtraLight,
                                            ..Default::default()
                                        }).height(height_single_line)
                                    // IDEA: Use ellipsis when part of next release
                                    .width(Fill).wrapping(Wrapping::WordOrGlyph)
                                ],
                                row![
                                    button("🗑").on_press(Message::TrashEntry(uuid)),
                                    button("✚").on_press(Message::SaveEntry(uuid)),
                                ]
                            ]).align_y(Alignment::Center)).style(style_only_border_box)
                            .padding(8)
                            .width(Fill).into(),
                        )
                    }))
                    .spacing(4)
                    .padding(4)
                )
                .height(Fill)
                .width(Fill)
                .spacing(8)
                .id("Rss-Items")
            )
            .align_x(Alignment::End)
            .style(style_only_border_box)
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

fn style_only_border_box(theme: &theme::Theme) -> container::Style {
    container::Style {
        border: border::Border {
            width: 1.0,
            radius: 5.0.into(),
            color: theme.extended_palette().background.weak.color,
        },
        ..container::Style::default()
    }
}

fn main() -> Result<(), iced::Error> {
    println!("Hello, world!");

    tracing_subscriber::fmt::init();
    iced::application(RssView::new, RssView::update, RssView::view)
        // .subscription(RssView::subscription)
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::Id;
    use iced_test::{Error, simulator};

    #[test]
    fn column_test() -> Result<(), Error> {
        let state = RssView::default();
        let mut ui = simulator(state.view());

        let _last: Result<Vec<_>, Error> = (1..=100)
            .map(|i| format!("Rss-Item-{i}"))
            .map(|string| ui.find(Id::from(string)))
            .collect();
        _last?;

        Ok(())
    }
}
