use crate::mnemonic;
use maud::html;
use maud::Markup;
use poly::browser;
use poly::browser::Capture;
use poly::browser::DomId;
use poly::browser::Effects;
use poly::page::Page;
use poly::page::PageMarkup;
use serde::{Deserialize, Serialize};
use url::Url;

const EXAMPLE_MNEMONIC: &'static str =
    "stove relax design safe deliver rigid height swamp know roof pitch innocent";

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
    pub result: Result<String, mnemonic::Error>,
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
            browser::on_submit(Id::Form, Msg::FormSubmitted),
            browser::on_click(Id::ShowExample, Msg::ShowExampleClicked),
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, AppEffect>, String> {
        match msg {
            Msg::MnemonicChanged(captured) => {
                model.mnemonic = captured.value();
                Ok(vec![])
            }

            Msg::FormSubmitted => {
                model.rows.push(Row {
                    mnemonic: model.mnemonic.clone(),
                    result: mnemonic::to_address(&model.mnemonic),
                });

                model.mnemonic = String::new();

                Ok(vec![])
            }

            Msg::ShowExampleClicked => {
                model.rows.push(Row {
                    mnemonic: EXAMPLE_MNEMONIC.to_string(),
                    result: mnemonic::to_address(EXAMPLE_MNEMONIC),
                });

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
    Form,
    ShowExample,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    MnemonicChanged(Capture<String>),
    FormSubmitted,
    ShowExampleClicked,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Seed phrase to address" }
        meta name="viewport" content="width=device-width, initial-scale=1";
        link rel="stylesheet" href="/app.css";
        script defer type="module" src="/home_page.js" {}
    }
}

fn view_body(model: &Model) -> maud::Markup {
    html! {
        div id=(Id::Seedphrase) class="sm:pt-8 mx-auto max-w-7xl sm:px-6 lg:px-8" {
            main class="divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow" {
                div class="px-4 py-5 sm:px-6" {
                    (view_input(model))
                    p class="mt-2" {
                        div class=" text-xs text-gray-500" {
                            "Enter a bip39 seed phrase (mnemonic) and you will get back it's first ethereum wallet address."
                        }
                        div class=" text-xs text-gray-500" {
                            "Although everything happens client-side you should never enter a seed phrase that contain any valuables."
                        }
                    }
                }

                div {
                    @if !model.rows.is_empty() {
                        (view_table(model))
                    } @else {
                        div class="py-8 flex items-center justify-center w-full" {
                            button id=(Id::ShowExample) class="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" type="button" {
                                "Show Example"
                            }
                        }
                    }
                }
            }


            footer {
                div class="mt-8 flex justify-between border-t border-gray-200 text-center text-xs text-gray-500 sm:text-left" {
                    span class="flex flex-col justify-end mt-4" {
                        "A glotlabs project."
                    }

                    span class="flex flex-col justify-end mt-4" {
                        a href="https://github.com/glotlabs/seedphrase" target="_blank" {
                            (github_icon())
                        }
                    }
                }
            }
        }
    }
}

fn github_icon() -> Markup {
    html! {
        svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" fill="currentColor" {
            path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" {}
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

                button class="whitespace-nowrap h-11 items-center ml-4 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" type="submit" {
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
                        div class="text-green-500 w-8 h-8" {
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
                        (mnemonic_error_to_string(err))
                    }
                    td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500" {
                        div class="text-red-500 w-8 h-8" {
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
            div {
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

fn mnemonic_error_to_string(err: &mnemonic::Error) -> String {
    match err {
        mnemonic::Error::InvalidPhrase => {
            // fmt
            format!("Error: not a valid seed phrase")
        }

        mnemonic::Error::InvalidWordCount(count) => {
            // fmt
            format!(
                "Error: seed phrase contains {} words, expected 12, 15, 18, 21 or 24 words",
                count
            )
        }

        mnemonic::Error::InvalidWord(word) => {
            // fmt
            format!("Error: '{}' is not a valid bip39 word", word)
        }

        mnemonic::Error::Internal(msg) => {
            // fmt
            format!("Internal error: {}", msg)
        }
    }
}

fn chunk_mnemonic(mnemonic: &str) -> Vec<String> {
    let parts: Vec<_> = mnemonic.split_whitespace().collect();
    parts.chunks(6).map(|chunk| chunk.join(" ")).collect()
}
