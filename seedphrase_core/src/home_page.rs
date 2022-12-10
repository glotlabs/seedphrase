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
    pub address: String,
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
            address: Default::default(),
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
                model.address = String::new();
                Ok(vec![])
            }

            Msg::CheckClicked => {
                model.address = mnemonic::to_address(&model.mnemonic);
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
        page::render_page_maud(markup)
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
    //html! {
    //    div id=(Id::Seedphrase) {
    //        input id=(Id::Mnemonic) value=(model.mnemonic) type="text" {}
    //        div class="flex p-4" {
    //            (model.address)
    //        }
    //    }
    //}

    html! {
        div id=(Id::Seedphrase) class="mt-5 md:col-span-2 md:mt-0" {
            form id=(Id::Form) {
                div class="overflow-hidden shadow sm:rounded-md" {
                    div class="bg-white px-4 py-5 sm:p-6" {
                        div class="grid grid-cols-6 gap-6" {
                            div class="col-span-6" {
                                label class="block text-sm font-medium text-gray-700" for=(Id::Mnemonic){
                                    "Seed phrase"
                                }
                                input id=(Id::Mnemonic) value=(model.mnemonic) class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" type="text" name="street-address" autocomplete="street-address";
                            }
                        }
                    }
                    div class="bg-gray-50 px-4 py-3 text-right sm:px-6" {
                        button id=(Id::Check) class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" type="submit" {
                            "Check"
                        }
                    }
                }
            }

           p {
                (model.address)
            }
        }
    }
}
