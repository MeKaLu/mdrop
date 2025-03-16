use iced::widget::{column, container, pick_list, slider, text};
use iced::{Center, Element, Fill, Theme};
use mdrop::filter::Filter;
use mdrop::gain::Gain;
use mdrop::indicator_state::IndicatorState;

const WIDTH: u16 = 300;

pub fn main() -> iced::Result {
    iced::application("mdrop", MdropGui::update, MdropGui::view)
        .theme(|_| Theme::CatppuccinMocha)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeVolume(u8),
    SelectFilter(Filter),
    SelectIndicator(IndicatorState),
    SelectGain(Gain),
}

pub struct MdropGui {
    value: u8,
    filter: Filter,
    indicator_state: IndicatorState,
    gain: Gain,
}

impl MdropGui {
    fn new() -> Self {
        MdropGui {
            value: 50,
            filter: Filter::default(),
            gain: Gain::default(),
            indicator_state: IndicatorState::default(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeVolume(value) => {
                self.value = value;
            }
            Message::SelectFilter(filter) => {
                self.filter = filter;
                println!("filter: {}", self.filter);
            }
            Message::SelectIndicator(indicator_state) => self.indicator_state = indicator_state,
            Message::SelectGain(gain) => self.gain = gain,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let filter_list =
            pick_list(&Filter::ALL[..], Some(self.filter), Message::SelectFilter).width(WIDTH);
        let gain_list =
            pick_list(&Gain::ALL[..], Some(self.gain), Message::SelectGain).width(WIDTH);
        let indicator_list = pick_list(
            &IndicatorState::ALL[..],
            Some(self.indicator_state),
            Message::SelectIndicator,
        )
        .width(WIDTH);
        let h_slider = container(
            slider(1..=100, self.value, Message::ChangeVolume)
                .default(50)
                .shift_step(5),
        )
        .width(WIDTH);

        let text = text(self.value);

        column![gain_list, indicator_list, filter_list, h_slider, text,]
            .width(Fill)
            .align_x(Center)
            .spacing(20)
            .padding(20)
            .into()
    }
}

impl Default for MdropGui {
    fn default() -> Self {
        Self::new()
    }
}
