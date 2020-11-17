use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct JsFontOptions {
    pub load_system_fonts: bool,
    pub font_files: Vec<String>,
    pub font_dirs: Vec<String>,
    pub default_font_family: String,
    pub default_font_size: f64,
    pub serif_family: String,
    pub sans_serif_family: String,
    pub cursive_family: String,
    pub fantasy_family: String,
    pub monospace_family: String,
}

impl JsFontOptions {
    pub fn new() -> JsFontOptions {
        JsFontOptions {
            load_system_fonts: true,
            font_files: vec![],
            font_dirs: vec![],
            default_font_family: "Times New Roman".to_string(),
            default_font_size: 12.0,
            serif_family: "Times New Roman".to_string(),
            sans_serif_family: "Arial".to_string(),
            cursive_family: "Comic Sans MS".to_string(),
            fantasy_family: "Impact".to_string(),
            monospace_family: "Courier New".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct JsOptions {
    pub path: Option<String>,
    pub font: JsFontOptions,
    pub dpi: f64,
    pub languages: Vec<String>,
    #[serde(deserialize_with = "deserialize_shape_rendering")]
    pub shape_rendering: usvg::ShapeRendering,
    #[serde(deserialize_with = "deserialize_text_rendering")]
    pub text_rendering: usvg::TextRendering,
    #[serde(deserialize_with = "deserialize_image_rendering")]
    pub image_rendering: usvg::ImageRendering,
    pub fit_to: JsFitTo,
    pub background: Option<String>,
}

impl JsOptions {
    pub fn new() -> JsOptions {
        JsOptions {
            path: None,
            font: JsFontOptions::new(),
            dpi: 96.0,
            languages: vec!["en".to_string()],
            shape_rendering: usvg::ShapeRendering::GeometricPrecision,
            text_rendering: usvg::TextRendering::OptimizeLegibility,
            image_rendering: usvg::ImageRendering::OptimizeQuality,
            fit_to: JsFitTo::Original,
            background: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JsFitTo {
    Original,
    Width { value: u32 },
    Height { value: u32 },
    Zoom { value: f32 },
}

fn deserialize_shape_rendering<'de, D>(deserializer: D) -> Result<usvg::ShapeRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::ShapeRendering::CrispEdges),
        1 => Ok(usvg::ShapeRendering::GeometricPrecision),
        2 => Ok(usvg::ShapeRendering::OptimizeSpeed),
        n => Err(serde::de::Error::custom(format_args!(
            "invalid shape_rendering value: {}, expected 0 through 2",
            n
        ))),
    }
}

fn deserialize_text_rendering<'de, D>(deserializer: D) -> Result<usvg::TextRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::TextRendering::GeometricPrecision),
        1 => Ok(usvg::TextRendering::OptimizeLegibility),
        2 => Ok(usvg::TextRendering::OptimizeSpeed),
        n => Err(serde::de::Error::custom(format_args!(
            "invalid shape_rendering value: {}, expected 0 through 2",
            n
        ))),
    }
}

fn deserialize_image_rendering<'de, D>(deserializer: D) -> Result<usvg::ImageRendering, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(usvg::ImageRendering::OptimizeQuality),
        1 => Ok(usvg::ImageRendering::OptimizeSpeed),
        n => Err(serde::de::Error::custom(format_args!(
            "invalid shape_rendering value: {}, expected 0 through 1",
            n
        ))),
    }
}