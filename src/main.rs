use chrono::{Datelike, NaiveDate, Utc};
use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;
use yew::prelude::Html;
use yew::{html, Component};

enum AppMsg {
    ToggleDate(NaiveDate),
}

struct App {
    data: HashMap<NaiveDate, bool>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        let data = LocalStorage::get("dates").unwrap_or_default();
        Self { data }
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ToggleDate(date) => {
                if self.data.get(&date).is_some() {
                    self.data.remove(&date);
                } else {
                    self.data.insert(date, true);
                }
                let _ = LocalStorage::set("dates", self.data.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        let now = Utc::now().naive_utc();
        let year = now.year();
        html! {
            <div class="container-xxl text-center">
                <table class="table table-borderless table-sm">
                    <thead>
                        <tr>
                            <th colspan=31>
                                {year}
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            (1..=12).map(|month|{
                                html!{
                                    <tr>
                                        {
                                            (1..=days_in_month(year, month)).map(|day| {
                                                let date = NaiveDate::from_ymd_opt(year, month, day as u32).unwrap();
                                                let class = if self.data.get(&date).is_some() {
                                                    "btn btn-success btn-circle btn-lg"
                                                } else {
                                                    "btn btn-light btn-circle btn-lg"
                                                };
                                                html!{
                                                    <td onclick={ctx.link().callback(move |_| AppMsg::ToggleDate(date))}>
                                                        <button type="button" {class} title={date.ordinal().to_string()}>{day}</button>
                                                    </td>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
            </div>
        }
    }
}

pub fn days_in_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days()
}

fn main() {
    yew::Renderer::<App>::new().render();
}