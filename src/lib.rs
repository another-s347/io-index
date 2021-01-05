use lang::Language;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use ybc::NavbarItemTag::{ A, Div };
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::NavbarFixed::Top;
use ybc::TileSize::Four;
use yewtil::future::LinkFuture;
use blog_tile::BlogTile;

mod lang;
mod blog_tile;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    language: Language
}

enum Msg {
    AddOne,
    ChangeLangEn,
    ChangeLangZh
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        yew::services::ConsoleService::log("new");
        Self {
            link,
            value: 0,
            language: Language::zh()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::ChangeLangEn => {
                yew::services::ConsoleService::log("change lang to en");
                self.language = Language::en()
            }
            Msg::ChangeLangZh => {
                yew::services::ConsoleService::log("change lang to zh");
                self.language = Language::zh()
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <ybc::Navbar navbrand=self.view_navbrand() navstart=self.view_navstart() navend=self.view_lang_drop() />
                // <ybc::Button onclick=self.link.callback(|x| Msg::AddOne)>{ "+1" }</ybc::Button>
                { self.view_blogs() }
            </div>
        }
    }
}

impl Model {
    fn view_navbrand(&self) -> Html {
        html!{
            <>
                <ybc::NavbarItem tag=A>
                    <img src="https://bulma.io/images/bulma-logo.png" />
                </ybc::NavbarItem>
            </>
        }
    }

    fn view_lang_link(&self) -> Html {
        html! {
            { self.language.nav_lang_link }
        }
    }

    fn view_lang_drop(&self) -> Html {
        html! {
            <ybc::NavbarDropdown navlink=self.view_lang_link() hoverable=true>
                <ybc::NavbarItem tag=A href="javascript:void(0)">
                    <div onclick=self.link.callback(|_|Msg::ChangeLangZh)>
                        { "中文" }
                    </div>
                </ybc::NavbarItem>
                <ybc::NavbarItem tag=A href="javascript:void(0)">
                    <div onclick=self.link.callback(|_|Msg::ChangeLangEn)>
                        { "English" }
                    </div>
                </ybc::NavbarItem>
            </ybc::NavbarDropdown>
        }
    }

    fn view_navdrop(&self) -> Html {
        html! {
            <ybc::NavbarDropdown navlink=self.view_navlink() hoverable=true>
                <ybc::NavbarItem tag=A>
                    { "About" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag=A>
                    { "Jobs" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag=A>
                    { "Contact" }
                </ybc::NavbarItem>
                <ybc::NavbarDivider />
                <ybc::NavbarItem tag=A>
                    { "Report an issue" }
                </ybc::NavbarItem>
            </ybc::NavbarDropdown>
        }
    }

    fn view_navend(&self) -> Html {
        html! {
            <ybc::NavbarItem tag=Div>
                // Create div container for button groups
                <ybc::Buttons>
                    // Button classes property accepts Option<String> type. `is-primary` here provides color styling. 
                    <ybc::Button classes=Some("is-primary")>
                        <strong>{ "Sign up" }</strong>
                    </ybc::Button>
                    <ybc::Button classes=Some("is-light")>
                        { "Log in" }
                    </ybc::Button>
                </ybc::Buttons>
            </ybc::NavbarItem>
        }
    }

    // Contruct the contents of the `navbar-link` section and return Html type that navlink property of NavbarDropdown expects.
    // Html type gets tossed into navlink field of NavbarDropdownProps struct. Consult ybc Docs for more info.
    fn view_navlink(&self) -> Html {
        html! {
            { "More" }
        }
    }

    // Contruct the contents of the `navbar-start` section and return Html type that navstart property of Navbar expects.
    // Html type gets tossed into navstart field of NavbarProps struct. Consult ybc Docs for more info.
    fn view_navstart(&self) -> Html {
        html! {
            <>
                <ybc::NavbarItem tag=A>
                    { self.language.nav_blogs }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag=A>
                    { self.language.nav_projects }
                </ybc::NavbarItem>
            </>
        }
    }

    fn view_blogs(&self) -> Html {
        html! {
            <>
            <ybc::Container fluid=true>
            <ybc::Tile ctx=Ancestor>
              <ybc::Tile ctx=Parent vertical=true>
                <ybc::Tile ctx=Child classes="box">
                <ybc::Title>{"Blogs"}</ybc::Title>
                  <img src="https://github.com/another-s347/md-pages/workflows/Auto-Deploy/badge.svg?event=push" />
                  <BlogTile lang={self.language.lang} />
                </ybc::Tile>
                /* .. snip .. more tiles here .. */
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}