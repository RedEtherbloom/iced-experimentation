use iced::{
    advanced::text::Wrapping,
    Alignment, Element, Fill, Font, Pixels, Subscription, border,
    font::Weight,
    widget::{column, container, keyed_column, row, scrollable, text, text::LineHeight, theme},
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
        const DEFAULT_ICED_FONT_SIZE: f32 = 16.0;
        let height_single_line = LineHeight::default().to_absolute(Pixels(DEFAULT_ICED_FONT_SIZE));
        column![
            text("This is the ControlBar placeholder"),
            container(
                scrollable(
                    keyed_column((1..=100).map(|i| {
                        (
                            uuid::Uuid::now_v7(),
                            row![
                                container(column![
                                    text!("Element number {i}").font(Font {
                                        weight: Weight::Semibold,
                                        ..Default::default()
                                    }),
                                    text!("This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder! This is a placeholder!")
                                        .font(Font {
                                            weight: Weight::ExtraLight,
                                            ..Default::default()
                                        }).height(height_single_line)
                                    .width(Fill).wrapping(Wrapping::WordOrGlyph)
                                ])
                                .style(style_only_border_box)
                                .padding(8)
                                .width(Fill)
                            ]
                            .into(),
                        )
                    }))
                    .spacing(4)
                    .padding(4)
                )
                .height(Fill)
                .width(Fill)
                .spacing(8)
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
