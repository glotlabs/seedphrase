use crate::mnemonic;
use maud::html;
use maud::Markup;
use poly::browser;
use poly::browser::Capture;
use poly::browser::DomId;
use poly::browser::Effects;
use poly::browser::Value;
use poly::page;
use poly::page::Page;
use poly::page::PageMarkup;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub mnemonic: String,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub mnemonic: String,
    pub result: Result<String, String>,
}

pub struct HomePage {
    pub current_url: Url,
}

impl Page<Model, Msg, AppEffect, Markup> for HomePage {
    fn id(&self) -> &'static dyn DomId {
        &Id::Seedphrase
    }

    fn init(&self) -> (Model, Effects<Msg, AppEffect>) {
        let model = Model {
            mnemonic: Default::default(),
            rows: Default::default(),
        };

        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, _model: &Model) -> browser::Subscriptions<Msg, AppEffect> {
        vec![
            browser::on_input(Id::Mnemonic, Msg::MnemonicChanged),
            //browser::on_click(Id::Check, Msg::CheckClicked),
            browser::on_submit(Id::Form, Msg::CheckClicked),
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, AppEffect>, String> {
        match msg {
            Msg::MnemonicChanged(captured) => {
                model.mnemonic = captured.value();
                Ok(vec![])
            }

            Msg::CheckClicked => {
                model.rows.push(Row {
                    mnemonic: model.mnemonic.clone(),
                    result: mnemonic::to_address(&model.mnemonic),
                });

                model.mnemonic = String::new();

                Ok(vec![])
            }
        }
    }

    fn view(&self, model: &Model) -> PageMarkup<Markup> {
        PageMarkup {
            head: view_head(),
            body: view_body(model),
        }
    }

    fn render(&self, markup: Markup) -> String {
        markup.into_string()
    }

    fn render_page(&self, markup: PageMarkup<Markup>) -> String {
        render_page(markup)
    }
}

#[derive(strum_macros::Display, poly_macro::DomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    Seedphrase,
    Mnemonic,
    Check,
    Form,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    MnemonicChanged(Capture<String>),
    CheckClicked,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Home Page" }
        link rel="stylesheet" href="/app.css";
        script defer type="module" src="/home_page.js" {}
    }
}

fn view_body(model: &Model) -> maud::Markup {
    html! {
        div id=(Id::Seedphrase) class="sm:pt-8 mx-auto max-w-7xl sm:px-6 lg:px-8" {
            div class="divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow" {
                div class="px-4 py-5 sm:px-6" {
                    (view_input(model))
                    p class="mt-2" {
                        div class=" text-xs text-gray-500" {
                            "Enter a bip39 mnemonic and you will get back it's first ethereum wallet address."
                        }
                        div class=" text-xs text-gray-500" {
                            "Although everything happens client-side you should never enter a seed phrase that contain any valuables."
                        }
                    }
                }

                div {
                    @if !model.rows.is_empty() {
                        (view_table(model))
                    }
                }
            }
        }

    }
}

fn view_input(model: &Model) -> Markup {
    html! {
        form id=(Id::Form) {
            label class="mb-1 block text-sm font-medium text-gray-700" for=(Id::Mnemonic) {
                "Seed phrase"
            }

            div class="flex items-center" {
                input id=(Id::Mnemonic) value=(model.mnemonic) class="text-xl block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="text";

                button id=(Id::Check) class="whitespace-nowrap h-11 items-center ml-4 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" type="submit" {
                    "Get Address"
                }
            }
        }
    }
}

fn view_table(model: &Model) -> Markup {
    html! {
        div class="px-4 sm:px-6 lg:px-8" {
            div class="flex flex-col" {
                div class="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8" {
                    div class="inline-block min-w-full py-2 align-middle" {
                        div class="overflow-hidden shadow-sm ring-1 ring-black ring-opacity-5" {
                            table class="min-w-full divide-y divide-gray-300" {
                                thead class="bg-gray-50" {
                                    tr {
                                        th class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6 lg:pl-8" scope="col" {
                                            "Seed phrase"
                                        }
                                        th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col" {
                                            "Address"
                                        }
                                        th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col" {
                                            "Status"
                                        }
                                    }
                                }
                                tbody class="divide-y divide-gray-200 bg-white" {
                                    @for row in &model.rows {
                                        (view_row(row))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn view_row(row: &Row) -> Markup {
    match &row.result {
        Ok(address) => {
            html! {
                tr {
                    td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm text-gray-500 sm:pl-6 lg:pl-8" {
                        (view_mnemonic(&row.mnemonic))
                    }
                    td class="whitespace-nowrap px-3 py-4 text-sm font-medium text-gray-900" {
                        (address)
                    }
                    td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500" {
                        div class="text-green-500 w-10 h-10" {
                            (heroicons_maud::check_circle_outline())
                        }
                    }
                }
            }
        }

        Err(err) => {
            html! {
                tr {
                    td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm text-gray-500 sm:pl-6 lg:pl-8" {
                        (view_mnemonic(&row.mnemonic))
                    }
                    td class="px-3 py-4 text-sm text-gray-500" {
                        (err)
                    }
                    td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500" {
                        div class="text-red-500 w-10 h-10" {
                            (heroicons_maud::exclamation_circle_outline())
                        }
                    }
                }
            }
        }
    }
}

fn view_mnemonic(mnemonic: &str) -> Markup {
    html! {
        @for chunk in chunk_mnemonic(mnemonic) {
            div class="" {
                (chunk)
            }
        }
    }
}

pub fn render_page(markup: PageMarkup<Markup>) -> String {
    (html! {
        (maud::DOCTYPE)
        html class="h-full bg-gray-100" {
            head {
                meta charset="utf-8";
                (markup.head)
            }
            body class="h-full" {
                (markup.body)
            }
        }
    })
    .into_string()
}

fn chunk_mnemonic(mnemonic: &str) -> Vec<String> {
    let parts: Vec<_> = mnemonic.split_whitespace().collect();
    parts.chunks(6).map(|chunk| chunk.join(" ")).collect()
}
