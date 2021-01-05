pub struct Language {
    pub lang: &'static str,
    pub nav_lang_link: &'static str,
    pub nav_blogs: &'static str,
    pub nav_projects: &'static str,
    pub nav_learning: &'static str
}

impl Language {
    pub fn from_lang(lang:&'static str) -> Self {
        match lang {
            "zh" => Self::zh(),
            "en" => Self::en(),
            _ => panic!("unknown lang")
        }
    }

    pub fn zh() -> Self {
        Self {
            lang: "zh",
            nav_lang_link: "语言",
            nav_blogs: "文章",
            nav_projects: "项目",
            nav_learning: "学习",
        }
    }

    pub fn en() -> Self {
        Self {
            lang: "en",
            nav_lang_link: "Language",
            nav_blogs: "Blogs",
            nav_projects: "Projects",
            nav_learning: "Learning",
        }
    }
}