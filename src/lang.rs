pub struct Language {
    pub lang: &'static str,
    pub nav_lang_link: &'static str,
    pub nav_blogs: &'static str,
    pub nav_projects: &'static str,
    pub nav_learning: &'static str,

    pub last_update: &'static str,
    pub time_week: &'static str,
    pub time_day: &'static str,
    pub time_hour: &'static str,
    pub time_minutes: &'static str,
    pub time_ago:&'static str,
    pub time_just: &'static str,
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
            time_week: "周",
            time_day: "天",
            time_hour: "小时",
            time_ago: "前",
            time_minutes: "分钟",
            time_just: "刚刚",
            last_update: "最后更新",
        }
    }

    pub fn en() -> Self {
        Self {
            lang: "en",
            nav_lang_link: "Language",
            nav_blogs: "Blogs",
            nav_projects: "Projects",
            nav_learning: "Learning",
            time_week: "weeks",
            time_day: "days",
            time_hour: "hours",
            time_ago: "ago",
            time_minutes: "minutes",
            time_just: "just now",
            last_update:"Last update"
        }
    }
}