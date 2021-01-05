use fetch::FetchTask;
use wasm_bindgen::prelude::*;
use ybc::NavbarFixed::Top;
use ybc::NavbarItemTag::{Div, A};
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::{services::ConsoleService, Properties};
use yew::{
    format::Nothing,
    prelude::*,
    services::fetch::Response,
    services::fetch::{self, FetchOptions},
};
use yewtil::future::LinkFuture;
use xml::reader::{EventReader, XmlEvent};
use chrono::prelude::*;

use crate::lang::Language;

#[derive(Debug)]
struct Blog {
    title: String,
    link: String,
    last_update: DateTime<Utc>
}

impl Default for Blog {
    fn default() -> Self {
        Self {
            title: String::new(),
            link: String::new(),
            last_update: Utc.timestamp_millis(0),
        }
    }
}

#[derive(Debug)]
struct Blogs {
    last_update: DateTime<Utc>,
    blogs: Vec<Blog>
}

pub struct BlogTile {
    link: ComponentLink<Self>,
    language: Language,
    blogs: BlogStatus,
    fetch_task: Option<FetchTask>,
    props: BlogProperty
}

#[derive(Properties, Clone)]
pub struct BlogProperty {
    #[prop_or("zh")]
    pub lang: &'static str,
}

pub enum BlogMessage {
    Done(Blogs),
    Error(String),
}

enum BlogStatus {
    Fetching,
    Done(Blogs),
    Err(String)
}

impl Component for BlogTile {
    type Message = BlogMessage;

    type Properties = BlogProperty;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = fetch_sitemap(&link, props.lang);

        Self {
            link,
            language: Language::from_lang(props.lang),
            blogs: BlogStatus::Fetching,
            fetch_task: Some(task),
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BlogMessage::Done(blogs) => {
                match &self.blogs {
                    BlogStatus::Done(b) => {
                        if b.last_update == blogs.last_update {
                            return false;
                        }
                    }
                    _ => {}
                }
                self.blogs = BlogStatus::Done(blogs);
            }
            BlogMessage::Error(err) => {
                self.blogs = BlogStatus::Err(err);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props.lang == _props.lang {
            true
        }
        else {
            self.props = _props;
            self.fetch_task = Some(fetch_sitemap(&self.link, self.props.lang));
            false
        }
    }

    fn view(&self) -> Html {
        match &self.blogs {
            BlogStatus::Fetching => {
                html! {
                    <>
                        <p>{"Fetching"}</p>
                    </>
                }
            }
            BlogStatus::Done(blogs) => {
                html! {
                    <>
                        <p>{blogs.last_update}</p>
                        { blogs.blogs.iter().map(render_blog).collect::<Html>() }
                    </>
                }
            }
            BlogStatus::Err(err) => {
                html! {
                    <>
                        <p>{err}</p>
                    </>
                }
            }
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn render_blog(blog: &Blog) -> Html {
    html! {
        <>
            <a href={blog.link.as_str()}>{&blog.title}</a>
        </>
    }
}

fn fetch_sitemap(link: &ComponentLink<BlogTile>, lang:&'static str) -> FetchTask {
    ConsoleService::log("fetch");
    ConsoleService::log(lang);
    let url = match lang {
        "en" => "https://another-s347.github.io/blogs-en/custom_sitemap.xml",
        "zh" => "http://another-s347.github.io/blogs/custom_sitemap.xml",
        _ => panic!("unknown lang")
    };
    let callback = link.callback(
        |response: Response<Result<String, anyhow::Error>>| -> BlogMessage {
            match response.body() {
                Ok(s) => {
                    match parse_sitemap_to_blog(s.as_str()) {
                        Ok(s) => {
                            BlogMessage::Done(s)
                        }
                        Err(err) => {
                            BlogMessage::Error(err.to_string())
                        }
                    }
                }
                Err(err) => {
                    BlogMessage::Error(err.to_string())
                }
            }
        },
    );
    let options = FetchOptions {
        // mode: Some(yew::web_sys::RequestMode::NoCors),
        ..FetchOptions::default()
    };
    let request = fetch::Request::get(url)
        .body(Nothing)
        .expect("build request failed");
    yew::services::FetchService::fetch_with_options(request, options, callback).expect("fetch failed")
}

enum Parser {
    Enter(Blog, SetState),
    Exit
}

enum SetState {
    Loc,
    LastUpdate,
    Title,
    None
}

impl Parser {
    pub fn new() -> Self {
        Parser::Exit
    }

    pub fn set(&mut self, data: String) {
        match self {
            Parser::Enter(blog, state) => {
                match std::mem::replace(state, SetState::None) {
                    SetState::Loc => {
                        blog.link = data;
                    }
                    SetState::LastUpdate => {
                        blog.last_update = Utc.timestamp_millis(data.parse().unwrap());
                    }
                    SetState::Title => {
                        blog.title = data;
                    }
                    SetState::None => {
                        panic!("invalid state")
                    }
                }
            }
            Parser::Exit => {
                panic!("invalid state")
            }
        }
    }

    pub fn enter(&mut self, state: SetState) {
        match self {
            Parser::Enter(blog, s) => {
                *s = state;
            }
            Parser::Exit => {
                *self = Parser::Enter(Blog::default(), state);
            }
        }
    }

    pub fn exit(&mut self) -> Option<Blog> {
        match std::mem::replace(self, Parser::Exit) {
            Parser::Enter(blog, state) => Some(blog),
            Parser::Exit => None,
        }
    }
}

fn parse_sitemap_to_blog(data: &str) -> anyhow::Result<Blogs> {
    let mut parser = Parser::new();
    let mut ret = vec![];
    for event in xml::reader::EventReader::new(data.as_bytes()) {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "loc" {
                    parser.enter(SetState::Loc);
                }
                if name.local_name == "lastmod" {
                    parser.enter(SetState::LastUpdate);
                }
                if name.local_name == "title" {
                    parser.enter(SetState::Title);
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "url" {
                    ret.push(parser.exit().ok_or(anyhow::anyhow!(""))?);
                }
            }
            XmlEvent::Characters(data) => {
                parser.set(data);
            }
            _ => {}
        }
    }

    let last_update = ret.iter().map(|x|x.last_update).max().unwrap_or(Utc.timestamp_millis(0));
    
    Ok(Blogs {
        last_update,
        blogs: ret,
    })
}

#[test]
fn test_parse_sitemap() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
      
      <url>
        <title>使用PyO3跨越编程语言的异步</title>
        <loc>http://another-s347.github.io/blogs/2021/01/01/async-pyo3-1/</loc>
        
        <lastmod>1609859438225</lastmod>
        
      </url>
      
      <url>
        <title></title>
        <loc>http://another-s347.github.io/blogs/2021/01/05/dummy/</loc>
        
        <lastmod>1609859415394</lastmod>
        
      </url>
      
    
      <!-- <url>
        <loc>http://another-s347.github.io/blogs</loc>
        <lastmod>2021-01-05</lastmod>
        <changefreq>daily</changefreq>
        <priority>1.0</priority>
      </url>
    
      
    
       -->
    </urlset>
    "#;
    println!("{:#?}",parse_sitemap_to_blog(xml));
}