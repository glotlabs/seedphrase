use seedphrase_core::home_page;
use poly::page::Page;
use std::env;
use url::Url;

fn main() {
    let args_: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_.iter().map(|s| s.as_ref()).collect();

    match args[1..] {
        ["home_page"] => {
            let page = home_page::HomePage {
                current_url: Url::parse("http://localhost/").unwrap(),
            };
            print_html(page);
        }

        _ => {
            println!("Invalid command");
        }
    }
}

fn print_html<Model, Msg, AppEffect, Markup>(page: impl Page<Model, Msg, AppEffect, Markup>) {
    let (model, _effects) = page.init();
    let markup = page.view(&model);
    println!("{}", page.render_page(markup));
}
