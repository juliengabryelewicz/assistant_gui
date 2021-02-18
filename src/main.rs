mod plugins;
mod style;

use chrono::prelude::*;
use iced::{
    button, pick_list, scrollable, Align, Application, Button, Column, 
    Command, Container, Element, HorizontalAlignment, PickList, Row, 
    Scrollable, Settings, Subscription, Text, text_input, 
    TextInput, time, VerticalAlignment};
use iced::Length;
use rss::Channel;
use plugins::{news, state, weather};


pub fn main() -> iced::Result {
    let mut settings_app = Settings::default();
    settings_app.window.size = (1024,600);
    Assistant::run(settings_app)
}

#[derive(Debug, Default)]
struct State {
    page_show: String,
    input_weatherapi: text_input::State,
    weatherapi_value: String,
    input_searchcity: text_input::State,
    searchcity_value: String,
    go_to_clock: button::State,
    go_to_meteo: button::State,
    go_to_news: button::State,
    go_to_parameter: button::State,
    dirty: bool,
    saving: bool,
    local_date: String,
    local_time: String,
    weather_json: String,
    rss_newspaper: Channel,
    pick_list: pick_list::State<news::Newspaper>,
    selected_newspaper: news::Newspaper,
    scroll: scrollable::State,
}

#[derive(Debug)]
enum Assistant {
    Loading,
    Loaded(State)
}

#[derive(Debug, Clone)]
enum Message {
    ClockPressed,
    MeteoPressed,
    NewsPressed,
    ParameterPressed,
    Saved(Result<(), state::SaveError>),
    Loaded(Result<state::SavedState, state::LoadError>),
    NewspaperSelected(news::Newspaper),
    SearchCityEdited(String),
    WeatherApiEdited(String),
    Tick(chrono::DateTime<chrono::Local>),
}

impl Application for Assistant {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Assistant::Loading,
            Command::perform(state::SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Assistant personnel")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Assistant::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Assistant::Loaded(State {
                            page_show: String::from("clock"),
                            weatherapi_value: state.weatherapi_value,
                            searchcity_value: state.searchcity_value,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Assistant::Loaded(State {
                            page_show: String::from("clock"),
                            ..State::default()
                        });
                    }
                    _ => {}
                }

                Command::none()
            }
            Assistant::Loaded(state) => {
                let mut saved = false;
                match message {
                    Message::ClockPressed => {
                        state.page_show = String::from("clock");
                    }
                    Message::MeteoPressed => {
                        let weather_response = weather::get_weather_from_search(&state.searchcity_value, &state.weatherapi_value);
                        state.weather_json = match weather_response {
                            Ok(v) => { v.to_string() },
                            Err(_e) => { json::parse(r#"{"error":"Vous avez besoin d'une clé API et d'une connexion Internet pour utiliser Météo"}"#).unwrap().to_string() }
                        };
                        state.page_show = String::from("meteo");
                    }
                    Message::NewsPressed => {

                        let newspaper_response = news::get_news(state.selected_newspaper);

                        state.rss_newspaper = match newspaper_response{
                            Ok(v) => { v },
                            Err(_e) => { Channel::default() }  
                        };
                        state.page_show = String::from("news");
                    }
                    Message::NewspaperSelected(newspaper) => {
                        state.selected_newspaper = newspaper;

                        let newspaper_response = news::get_news(newspaper);

                        state.rss_newspaper = match newspaper_response{
                            Ok(v) => { v },
                            Err(_e) => { Channel::default() }  
                        };
                    }
                    Message::ParameterPressed => {
                        state.page_show = String::from("parameter");
                    }
                    Message::SearchCityEdited(new_searchcity_value) => {
                        state.searchcity_value = new_searchcity_value;
                    }
                    Message::Tick(new_local_datetime) => {
                        state.local_date = new_local_datetime.format_localized("%A %e %B %Y", Locale::fr_FR).to_string();
                        state.local_time = new_local_datetime.format_localized("%T", Locale::fr_FR).to_string();
                    }
                    Message::WeatherApiEdited(new_weatherapi_value) => {
                        state.weatherapi_value = new_weatherapi_value;
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }
                    _ => {}
                }


                if !saved {
                    state.dirty = true;
                }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;
                    Command::perform(
                        state::SavedState {
                            weatherapi_value: state.weatherapi_value.clone(),
                            searchcity_value: state.searchcity_value.clone(),
                        }.save(),
                        Message::Saved,
                    )
                }else{
                    Command::none()
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(500))
            .map(|_| Message::Tick(chrono::Local::now()))
    }

    fn view(&mut self) -> Element<Message> {

        match self {
            Assistant::Loading => Row::new().push(Text::new("En cours de chargement...")).into(),
            Assistant::Loaded(State {
                page_show,
                input_weatherapi,
                weatherapi_value,
                input_searchcity,
                searchcity_value,
                go_to_clock,
                go_to_meteo,
                go_to_news,
                go_to_parameter,
                local_date,
                local_time,
                weather_json,
                rss_newspaper,
                pick_list,
                selected_newspaper,
                scroll,
                ..
            }) => {
                let news_button =
                Button::new(go_to_news, Text::new("Actualités").horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center))
                    .min_width(120)
                    .min_height(150)
                    .style(style::Button::News)
                    .on_press(Message::NewsPressed);
            
                let clock_button =
                    Button::new(go_to_clock, Text::new("Horloge").horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center))
                        .min_width(120)
                        .min_height(150)
                        .style(style::Button::Clock)
                        .on_press(Message::ClockPressed);
        
                let meteo_button =
                    Button::new(go_to_meteo, Text::new("Meteo").horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center))
                        .min_width(120)
                        .min_height(150)
                        .style(style::Button::Meteo)
                        .on_press(Message::MeteoPressed);
        
                let parameter_button =
                    Button::new(go_to_parameter, Text::new("Paramètres").horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center))
                        .min_width(120)
                        .min_height(150)
                        .style(style::Button::Parameters)
                        .on_press(Message::ParameterPressed);
        
                let content: Element<_> = match page_show.as_str() {
                    "clock" => {
                        Column::new()
                        .width(Length::Units(900))
                        .height(Length::Units(600))
                        .align_items(Align::Center)
                        .push(Row::new()
                        .align_items(Align::Center)
                        .height(Length::Units(300))
                        .push(Text::new(
                            &*local_time
                        ).size(150)))
                        .push(Row::new()
                        .align_items(Align::Center)
                        .height(Length::Units(300))
                        .push(Text::new(
                            &*local_date
                        )))
                        .into()
                    },
                    "meteo" => {
                        let weather_json_parse = json::parse(&weather_json).unwrap();
                        if weather_json_parse["cod"]==401 {
                            Column::new()
                            .width(Length::Units(900))
                            .height(Length::Units(600))
                            .spacing(20)
                            .push(Text::new("Clé API invalide ou ville manquante. Veuillez vérifier vos paramètres"))
                            .into()
                        }
                        else if !weather_json_parse["error"].is_string() {
                            let temperature = weather::calculate_temperature(weather_json_parse["main"]["temp"].to_owned().as_f32().unwrap()).to_string();
                            let temperature_min = weather::calculate_temperature(weather_json_parse["main"]["temp_min"].to_owned().as_f32().unwrap()).to_string();
                            let temperature_max = weather::calculate_temperature(weather_json_parse["main"]["temp_max"].to_owned().as_f32().unwrap()).to_string();
                            Column::new()
                            .width(Length::Units(900))
                            .height(Length::Units(600))
                            .padding(20)
                            .push(Row::new()
                            .push(Column::new()
                            .width(Length::Units(450))
                            .spacing(100)
                            .push(Text::new(["Temp. :".to_string(), temperature, "°C".to_string()].join(" ")).size(35))
                            .push(Text::new(["Min. :".to_string(), temperature_min, "°C".to_string()].join(" ")).size(35))
                            .push(Text::new(["Max. :".to_string(), temperature_max, "°C".to_string()].join(" ")).size(35))
                            )
                            .push(Column::new()
                            .width(Length::Units(450))
                            .spacing(100)
                            .push(Text::new(&weather_json_parse["name"].to_owned().to_string()).size(35))
                            .push(Text::new(&weather_json_parse["weather"][0]["main"].to_owned().to_string()).size(35))
                        )
                            )
                            .into()
                        }else{
                            Column::new()
                            .width(Length::Units(900))
                            .height(Length::Units(600))
                            .spacing(20)
                            .push(Text::new("Meteo a besoin d'une connexion internet et d'une clé API pour fonctionner"))
                            .into()
                        }

                    },
                    "news" => {

                        let pick_list_gui = PickList::new(
                            pick_list,
                            &news::Newspaper::ALL[..],
                            Some(*selected_newspaper),
                            Message::NewspaperSelected,
                        );

                        let news =  rss_newspaper.items()
                                .iter()
                                .enumerate()
                                .fold(Column::new().spacing(10).padding(20), |column, (_i, item)| {
                                    column.push(Text::new(item.title().unwrap_or("...")).size(35))
                                    .push(Text::new(item.pub_date().unwrap_or("")).size(15))
                                    .push(Text::new(item.description().unwrap_or("")).size(20))
                                });

                        Column::new()
                        .padding(20)
                        .push(pick_list_gui)
                        .push(Container::new(Scrollable::new(scroll).push(Container::new(news)).style(style::Scrollable).padding(10)
                        .width(Length::Fill)
                        .height(Length::Fill)
                    )).into()
                    },
                    "parameter" => {

                        let text_input_openweather = TextInput::new(
                            input_weatherapi,
                            "API Openweather",
                            weatherapi_value,
                            Message::WeatherApiEdited,
                        )
                        .padding(10)
                        .style(style::TextInput);

                        let text_input_searchcity = TextInput::new(
                            input_searchcity,
                            "Ville à chercher",
                            searchcity_value,
                            Message::SearchCityEdited,
                        )
                        .padding(10)
                        .style(style::TextInput);
                
                        Column::new()
                        .push(Text::new("Paramètres").size(50))
                        .padding(20)
                        .spacing(10)
                        .push(Text::new(
                            "API Openweather",
                        ))
                        .push(text_input_openweather)
                        .push(Text::new(
                            "Ville à chercher",
                        ))
                        .push(text_input_searchcity)
                        .into()
                    },
                    _ => { Column::new().into()}
                };
            
                let menu = Column::new()
                    .align_items(Align::Center)
                    .push(news_button)
                    .push(clock_button)
                    .push(meteo_button)
                    .push(parameter_button);
        
                Container::new(Row::new()
                .push(menu)
                .push(content))
                .style(style::Container)
                .into()
            }
        }
    }
}