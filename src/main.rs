mod svg;

use chrono::{Datelike, NaiveDate, Utc};
use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;
use std::collections::HashSet;
use svg::ArrowLeft;
use svg::ArrowRight;
use yew::prelude::Html;
use yew::{classes, html, Component};

enum AppMsg {
    SetYear(i32),
    ToggleDate(NaiveDate),
}

struct App {
    now: NaiveDate,
    year: i32,
    years: HashSet<i32>,
    data: HashMap<NaiveDate, bool>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        let data: HashMap<NaiveDate, bool> = LocalStorage::get("dates").unwrap_or_default();
        let years = data.keys().map(|day| day.year()).collect::<HashSet<i32>>();
        let now = Utc::now().naive_utc().date();
        let year = now.year();
        Self {
            now,
            year,
            years,
            data,
        }
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::SetYear(year) => {
                self.year = year;
                true
            }
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
        let last_year = self.year.saturating_sub(1);
        let next_year = self.year.saturating_add(1);
        html! {
            <div class={classes!("container")}>
                <table>
                    <thead>
                        <tr>
                            <th colspan=31>
                                <div class={classes!("title-bar")}>
                                    <div class={classes!("year-selector")}>
                                        if self.years.contains(&last_year) {
                                            <div onclick={ctx.link().callback(move |_| AppMsg::SetYear(last_year))} class={classes!("clickable")}>
                                                <ArrowLeft />
                                            </div>
                                        }
                                    </div>
                                    <div class={classes!("title")}>
                                        {self.year}
                                    </div>
                                    <div class={classes!("year-selector")}>
                                    if self.years.contains(&self.year.saturating_add(1)) {
                                        <div onclick={ctx.link().callback(move |_| AppMsg::SetYear(next_year))} class={classes!("clickable")}>
                                            <ArrowRight />
                                        </div>
                                    }
                                    </div>
                                </div>
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                    for month in 1..=12 {
                        <tr>
                            {
                                (1..=days_in_month(self.year, month)).map(|day| {
                                    let date = NaiveDate::from_ymd_opt(self.year, month, day as u32).unwrap();
                                    let mut class = classes!("day", "clickable");
                                    // Color of circle
                                    if self.data.get(&date).is_some() {
                                        class.push("selected");
                                    }
                                    // Border for today
                                    if date.ordinal() == self.now.ordinal() && date.year() == self.now.year() {
                                        class.push("today");
                                    }
                                    html!{
                                        <td onclick={ctx.link().callback(move |_| AppMsg::ToggleDate(date))}>
                                            <div {class}>
                                                {day}
                                            </div>
                                        </td>
                                    }
                                }).collect::<Html>()
                            }
                        </tr>
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
