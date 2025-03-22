use iced::widget::{column, container, pick_list, slider, text};
use iced::{Center, Element, Fill, Size, Theme};
use mdrop::filter::Filter;
use mdrop::gain::Gain;
use mdrop::indicator_state::IndicatorState;
use mdrop::volume::Volume;
use mdrop::{Moondrop, MoondropInfo};

const WIDTH: u32 = 300;

pub fn main() -> iced::Result {
    iced::application("mdrop", MdropGui::update, MdropGui::view)
        .window(iced::window::Settings {
            size: Size::new(300.0, 300.0),
            min_size: Some(Size::new(300.0, 300.0)),
            ..Default::default()
        })
        .theme(|_| Theme::CatppuccinMocha)
        // .run_with(move || MdropGui::new())
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    SetVolume,
    VolumeChanged(u32),
    SelectFilter(Filter),
    SelectIndicator(IndicatorState),
    SelectGain(Gain),
}

pub struct MdropGui {
    moondrop: Moondrop,
    info: MoondropInfo,
}

impl MdropGui {
    fn update(&mut self, message: Message) {
        match message {
            Message::SetVolume => {
                println!("final: {}", self.info.volume);
            }
            Message::VolumeChanged(value) => {
                self.info.volume = Volume::new(value);
            }
            Message::SelectFilter(filter) => {
                self.info.filter = filter;
                self.moondrop.set_filter(filter);
            }
            Message::SelectIndicator(indicator_state) => {
                self.info.indicator_state = indicator_state;
                self.moondrop.set_indicator_state(indicator_state);
            }
            Message::SelectGain(gain) => {
                self.info.gain = gain;
                self.moondrop.set_gain(gain);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let name = text(&self.info.name);

        let filter_list = pick_list(
            &Filter::ALL[..],
            Some(self.info.filter),
            Message::SelectFilter,
        )
        .width(WIDTH);
        let gain_list =
            pick_list(&Gain::ALL[..], Some(self.info.gain), Message::SelectGain).width(WIDTH);
        let indicator_list = pick_list(
            &IndicatorState::ALL[..],
            Some(self.info.indicator_state),
            Message::SelectIndicator,
        )
        .width(WIDTH);
        let h_slider = container(
            slider(1..=100, self.info.volume.inner(), Message::VolumeChanged)
                .on_release(Message::SetVolume), // .shift_step(1),
        )
        .width(WIDTH);

        let text = text(self.info.volume.inner());

        column![name, gain_list, indicator_list, filter_list, h_slider, text,]
            .width(Fill)
            .align_x(Center)
            .spacing(20)
            .padding(20)
            .into()
    }
}

impl Default for MdropGui {
    fn default() -> Self {
        let moondrop = Moondrop::new();
        let info = moondrop.get_all();
        Self { moondrop, info }
    }
}
