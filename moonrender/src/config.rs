use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    pub font: String,
    pub color: Color,
    pub size: f64,
    pub line_spacing: f64,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct HeadingSection {
    pub font: String,
    pub color: Color,

    pub level1_size: f64,
    pub level2_size: f64,

    pub line_spacing: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ListSection {
    pub font: String,
    pub color: Color,
    pub bullet: String,

    pub size: f64,
    pub bullet_padding: f64,

    pub line_spacing: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    pub margin_percent: f64,
    pub paragraph_spacing: f64,

    pub background_color: Color,

    pub content: Section,
    pub link: Section,
    pub monospace: Section,
    pub heading: HeadingSection,
    pub list: ListSection,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            margin_percent: 0.15,
            paragraph_spacing: 4.0,

            background_color: Color(255, 255, 255),

            content: Section {
                font: "sans-serif".to_owned(),
                color: Color(0, 0, 0),
                size: 13.5,
                line_spacing: 2.5,
            },

            link: Section {
                font: "sans-serif".to_owned(),
                color: Color(0, 0, 255),
                size: 13.5,
                line_spacing: 2.5,
            },

            monospace: Section {
                font: "monospace".to_owned(),
                color: Color(0, 0, 0),
                size: 12.0,
                line_spacing: 1.0,
            },

            heading: HeadingSection {
                font: "sans-serif".to_owned(),
                color: Color(0, 0, 0),

                level1_size: 27.0,
                level2_size: 20.0,

                line_spacing: 1.5,
            },

            list: ListSection {
                bullet: "•".to_owned(),
                color: Color(0, 0, 0),
                font: "sans-serif".to_owned(),
                size: 13.5,
                bullet_padding: 4.0,
                line_spacing: 2.5,
            },
        }
    }
}
